#![allow(dead_code)]
use std::{ffi::{CString, c_char, c_int, c_void}, ptr};
use std::thread;
use std::time::Duration;

unsafe extern "C" {
	fn xdl_open(filename: *const c_char, flags: c_int) -> *const c_void;
	fn xdl_close(handle: *const c_void);
	fn xdl_sym(handle: *const c_void, sym: *const c_char, sym_size: *mut usize) -> *mut c_void;
	fn xdl_dsym(handle: *const c_void, sym: *const c_char, sym_size: *mut usize) -> *mut c_void;
}

pub struct Xdl {
	handle: *const c_void,
}

impl Drop for Xdl {
	fn drop(&mut self) {
		unsafe { xdl_close(self.handle); }
	}
}

impl Xdl {
	pub fn open(filename: &str, flags: u32) -> Option<Self> {
		let c_filename = CString::new(filename).ok()?;

		let handle = unsafe { xdl_open(c_filename.as_ptr(), flags as c_int) };

		if handle.is_null() { None } else { Some(Self { handle }) }
	}

	pub fn open_poll(filename: &str, flags: u32, max_attempts: u32) -> Option<Self> {
		for _ in 0..max_attempts {
			if let Some(hndl) = Self::open(filename, flags) {
				return Some(hndl);
			}
			thread::sleep(Duration::from_millis(10));
		}
		None
	}

	/// Search in .symtab
	pub fn sym(&self, sym: &str, sym_size: Option<&mut usize>) -> Option<*mut c_void> {
		let c_sym = CString::new(sym).ok()?;
		let siz_ptr = sym_size.map_or(ptr::null_mut(), |r| r as *mut usize);
		let addr = unsafe {
			xdl_sym(self.handle, c_sym.as_ptr(), siz_ptr)
		};

		if addr.is_null() { None } else { Some(addr) }
	}

	/// Search in .dynsym
	pub fn dsym(&self, sym: &str, sym_size: Option<&mut usize>) -> Option<*mut c_void> {
		let c_sym = CString::new(sym).ok()?;
		let siz_ptr = sym_size.map_or(ptr::null_mut(), |r| r as *mut usize);
		let addr = unsafe {
			xdl_dsym(self.handle, c_sym.as_ptr(), siz_ptr)
		};
			
		if addr.is_null() { None } else { Some(addr) }
	}
}
