#![allow(unsafe_op_in_unsafe_fn, dead_code)]
use std::ffi::{c_void, c_int};
use std::ptr;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;

const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void; // (void*)-1

unsafe extern "C" {
	fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, offset: i64) -> *mut c_void;
	fn munmap(addr: *mut c_void, len: usize) -> c_int;
	fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
	fn __clear_cache(begin: *mut c_void, end: *mut c_void);
}

static mut G_BRIDGE_POOL: *mut u8 = ptr::null_mut();
static mut G_BRIDGE_BYTES_USED: usize = 0;

unsafe fn _alloc_page_near(target_addr: usize) -> *mut c_void {
	let aligned_target = target_addr & !0xFFF_usize;
	const MAX_ARM64_BRANCH_RANGE: usize = 120 * 1024 * 1024;

	let mut offset: usize = 0x1000000;
	while offset < 0x7000000 {
		let mut hint_addr = aligned_target.wrapping_add(offset);
		let mut page = mmap(
			hint_addr as *mut c_void,
			4096,
			PROT_READ | PROT_WRITE,
			MAP_PRIVATE | MAP_ANONYMOUS,
			-1,
			0,
		);

		if page != MAP_FAILED {
			let allocated = page as usize;
			let distance = allocated.abs_diff(target_addr);
			if distance < MAX_ARM64_BRANCH_RANGE {
				return page;
			}
			munmap(page, 4096);
		}

		hint_addr = aligned_target.wrapping_sub(offset);
		page = mmap(
			hint_addr as *mut c_void,
			4096,
			PROT_READ | PROT_WRITE,
			MAP_PRIVATE | MAP_ANONYMOUS,
			-1,
			0,
		);

		if page != MAP_FAILED {
			let allocated = page as usize;
			let distance = allocated.abs_diff(target_addr);
			if distance < MAX_ARM64_BRANCH_RANGE {
				return page;
			}
			munmap(page, 4096);
		}

		offset += 0x100000;
	}

	ptr::null_mut()
}

unsafe fn _alloc_near(target: usize, size: usize) -> *mut c_void {
	let aligned_size = (size + 7) & !7_usize;

	if G_BRIDGE_POOL.is_null() || (G_BRIDGE_BYTES_USED + aligned_size > 4096) {
		G_BRIDGE_POOL = _alloc_page_near(target) as *mut u8;
		G_BRIDGE_BYTES_USED = 0;
	}

	if G_BRIDGE_POOL.is_null() {
		return ptr::null_mut();
	}

	let ptr = G_BRIDGE_POOL.add(G_BRIDGE_BYTES_USED) as *mut c_void;
	G_BRIDGE_BYTES_USED += aligned_size;
	ptr
}

/// Must be called on 12 byte stubs
pub unsafe fn _vk_hook_stub(
	address: *mut c_void,
	replace_call: *mut c_void,
	origin_call: *mut *mut c_void,
) -> i32 {
	let insn = address as *mut u32;
	let mut bti_c = false;

	// Check `BTI c`
	if *insn == 0xD503245F {
		bti_c = true;
	}

	let mut target = address as usize;
	let bridge = _alloc_near(target, 36);
	if bridge.is_null() { return 2; }

	let bridge_pg = (bridge as usize & !0xFFF_usize) as *mut c_void;
	mprotect(bridge_pg, 4096, PROT_READ | PROT_WRITE | PROT_EXEC);

	let mut bridge_code = bridge as *mut u32;
	/*
	 * [BTI c]
	 * LDR X16, [PC, #8]
	 * BR X16
	 * .quad replace_call
	 */
	if bti_c {
		*bridge_code = 0xD503245F;
		bridge_code = bridge_code.add(1);
	}
	*bridge_code = 0x58000050;
	bridge_code = bridge_code.add(1);
	*bridge_code = 0xD61F0200;
	bridge_code = bridge_code.add(1);

	*(bridge_code as *mut usize) = replace_call as usize;
	bridge_code = bridge_code.add(2);

	let copy_size = if bti_c { 16 } else { 12 };
	let orig_code = bridge_code;
	ptr::copy_nonoverlapping(address as *const u8, orig_code as *mut u8, copy_size);

	mprotect(bridge_pg, 4096, PROT_READ | PROT_EXEC);
	*origin_call = orig_code as *mut c_void;

	if bti_c {
		target += 4;
	}

	let offset = (bridge as isize) - (target as isize);
	let b_instruction = 0x14000000 | (((offset >> 2) & 0x03FFFFFF) as u32);

	let page_start = (target & !0xFFF_usize) as *mut c_void;
	mprotect(page_start, 4096, PROT_READ | PROT_WRITE | PROT_EXEC);

	*(target as *mut u32) = b_instruction;

	mprotect(page_start, 4096, PROT_READ | PROT_EXEC);
	__clear_cache(target as *mut c_void, (target + 4) as *mut c_void);
	__clear_cache(bridge, (bridge as usize + 64) as *mut c_void);

	0
}

/// Must be called on 12 byte exported stub. it hooks it and their api
pub unsafe fn _vk_hook_stub2(
	address: *mut c_void,
	replace_call: *mut c_void,
	origin_call: *mut *mut c_void,
) -> i32 {
	let api_addr = _vk_find_api12(address);

	let ret = _vk_hook_stub(address, replace_call, origin_call);
	if ret != 0 { return ret; }

	let ret = _vk_hook_stub(api_addr, replace_call, origin_call);
	if ret != 0 { return ret; }

	0
}

/// Must be called on 4 byte exported stubs in libvulkan.so
/// ```
/// B xxx
/// ```
/// or
/// ```
/// BTI c
/// B xxx
/// ```
pub unsafe fn _vk_find_api4(addr: *mut c_void) -> *mut c_void {
	if addr.is_null() { return ptr::null_mut(); }

	let mut insn = addr as *mut u32;

	if *insn == 0xD503245F {
		insn = insn.add(1);
	}
	if (*insn & 0xFC000000) != 0x14000000 { return ptr::null_mut(); }

	let offset26 = *insn & 0x03FFFFFF;
	let offset = ((offset26 << 6) as i32) >> 6;

	insn.offset(offset as isize) as *mut c_void
}

/// Must be called on 12 byte exported stubs in libvulkan.so
/// Note: api is same stub, but get called by game
/// ```
/// LDR x?? [x0]
/// LDR x?? [x??, #??]
/// BR  x??
/// ```
/// or
/// ```
/// BTI c
/// LDR x?? [x0]
/// LDR x?? [x??, #??]
/// BR  x??
/// ```
pub unsafe fn _vk_find_api12(addr: *mut c_void) -> *mut c_void {
	if addr.is_null() { return ptr::null_mut(); }

	let mut insn = addr as *mut u32;

	if *insn == 0xD503245F {
		insn = insn.add(1);
	}

	if (*insn & 0x3FC00000) != 0x39400000 { return ptr::null_mut(); }
	insn = insn.add(1);
	if (*insn & 0x3FC00000) != 0x39400000 { return ptr::null_mut(); }
	insn = insn.add(1);
	if (*insn & 0xFFFFFC1F) != 0xD61F0000 { return ptr::null_mut(); }
	insn = insn.add(1);

	insn as *mut c_void
}
