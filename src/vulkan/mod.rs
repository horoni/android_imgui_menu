#![allow(unsafe_op_in_unsafe_fn)]

mod utils;
pub mod types;

use std::ffi::{CStr, c_char, c_void};
use std::{ptr, thread};
use std::sync::{OnceLock, RwLock, RwLockWriteGuard, RwLockReadGuard};
use std::time::Duration;
use crate::{imgui, xdl};
use crate::vulkan::types::*;
use crate::and64inlinehook::a64_hook_function;

#[derive(Debug)]
pub struct VkState {
	pub instance: VkInstance,
	pub phys_dev: VkPhysicalDevice,
	pub dev: VkDevice,
	pub queue: VkQueue,
	pub queue_family: u32,
	pub descriptor_pool: VkDescriptorPool,
	pub command_pool: VkCommandPool,
	pub render_pass: VkRenderPass,
	pub img_index: u32,
	pub img_min: u32,
	pub imgui_inited: bool,
	pub screen_extent: VkExtent2D,
	pub framebuffers: Vec<VkFramebuffer>,
	pub cmd_bufs: [VkCommandBuffer; 8],
	pub fences: [VkFence; 8],
}
impl VkState {
	pub const fn new() -> Self {
		Self {
			instance: GeneralPtr::null(),
			phys_dev: GeneralPtr::null(),
			dev: GeneralPtr::null(),
			queue: GeneralPtr::null(),
			queue_family: 0,
			descriptor_pool: GeneralPtr::null(),
			command_pool: GeneralPtr::null(),
			render_pass: GeneralPtr::null(),
			img_index: 0,
			img_min: 0,
			imgui_inited: false,
			screen_extent: VkExtent2D {
				width: 0,
				height: 0,
			},
			framebuffers: Vec::new(),
			cmd_bufs: [GeneralPtr::null(); 8],
			fences: [GeneralPtr::null(); 8],
		}
	}
}
// SAFETY: Vulkan is safe between threads. Also RwLock is used.
unsafe impl Send for VkState {}
unsafe impl Sync for VkState {}

macro_rules! define_vk_fn_pool {
	($struct_name:ident { $($name:ident: $type:ty),* $(,)? }) => {
		#[derive(Debug)]
		pub struct $struct_name {
			$(
				pub $name: OnceLock<$type>,
			)*
		}

		impl $struct_name {
			pub const fn new() -> Self {
				Self {
					$(
						$name: OnceLock::new(),
					)*
				}
			}

			$(
				#[inline(always)]
				pub fn $name(&self) -> $type {
					*self.$name.get().unwrap_or_else(|| {
						panic!("Vulkan: Pfn '{}' is not initialized!", stringify!($name))
					})
				}
			)*
		}
	};
}

define_vk_fn_pool! {
	VkOrig {
		gipa: PfnVkGetInstanceProcAddr,
		gdpa: PfnVkGetDeviceProcAddr,
		cd:   PfnVkCreateDevice,
		ci:   PfnVkCreateInstance,
		ani:  PfnVkAcquireNextImageKHR,
		cs:   PfnVkCreateSwapchainKHR,
		cf:   PfnVkCreateFramebuffer,
		df:   PfnVkDestroyFramebuffer,
		gdq:  PfnVkGetDeviceQueue,
		qp:   PfnVkQueuePresentKHR,
	}
}

define_vk_fn_pool! {
	VkApi {
		ccp: PfnVkCreateCommandPool,
		cdp: PfnVkCreateDescriptorPool,
		qs:  PfnVkQueueSubmit,
		acb: PfnVkAllocateCommandBuffers,
		bcb: PfnVkBeginCommandBuffer,
		ecb: PfnVkEndCommandBuffer,
		brp: PfnVkCmdBeginRenderPass,
		erp: PfnVkCmdEndRenderPass,
		cff: PfnVkCreateFence,
		wff: PfnVkWaitForFences,
		rff: PfnVkResetFences,
	}
}

static VK_ORIG: VkOrig = VkOrig::new();
static VK_API: VkApi = VkApi::new();
static VK_STATE: RwLock<VkState> = RwLock::new(VkState::new());

fn get_state_mut() -> RwLockWriteGuard<'static, VkState> {
	VK_STATE.write().unwrap()
}
pub fn get_state() -> RwLockReadGuard<'static, VkState> {
	VK_STATE.read().unwrap()
}

macro_rules! hook_vk_export {
	($lib:expr, $name:expr, $hook_fn:expr, $orig_store:expr) => {
		let addr = $lib.sym($name, None).unwrap_or(ptr::null_mut());
		if !addr.is_null() {
			let mut orig: *mut c_void = ptr::null_mut();
			unsafe {
				let ret = utils::_vk_hook_stub2(addr, $hook_fn as *mut c_void, &mut orig);
				trace!("HOOK: {} {:?} {:?} {}", $name, utils::_vk_find_api(addr), orig, ret);
				if !orig.is_null() {
					$orig_store.set(std::mem::transmute(orig)).ok();
				}
			}
		} else {
			trace!("HOOK: {} sym is null", $name);
		}
	};
}

pub fn init() {
	let lib_vulkan = loop {
		if let Some(hndl) = xdl::Xdl::open("libvulkan.so", 0) {
			break hndl;
		}
		thread::sleep(Duration::from_millis(10));
	};

	hook_vk_export!(lib_vulkan, "vkGetInstanceProcAddr", vk_gipa_hook, VK_ORIG.gipa);
	hook_vk_export!(lib_vulkan, "vkGetDeviceProcAddr", vk_gdpa_hook, VK_ORIG.gdpa);
	hook_vk_export!(lib_vulkan, "vkCreateInstance", vk_ci_hook, VK_ORIG.ci);
	hook_vk_export!(lib_vulkan, "vkCreateDevice", vk_cd_hook, VK_ORIG.cd);
	hook_vk_export!(lib_vulkan, "vkCreateFramebuffer", vk_cf_hook, VK_ORIG.cf);
	hook_vk_export!(lib_vulkan, "vkDestroyFramebuffer", vk_df_hook, VK_ORIG.df);
	hook_vk_export!(lib_vulkan, "vkGetDeviceQueue", vk_gdq_hook, VK_ORIG.gdq);
	hook_vk_export!(lib_vulkan, "vkAcquireNextImageKHR", vk_ani_khr_hook, VK_ORIG.ani);
	hook_vk_export!(lib_vulkan, "vkCreateSwapchainKHR", vk_cs_khr_hook, VK_ORIG.cs);
	hook_vk_export!(lib_vulkan, "vkQueuePresentKHR", vk_qp_khr_hook, VK_ORIG.qp);
}

unsafe extern "C" fn vk_gipa_hook(instance: VkInstance, name: *const c_char) -> *mut c_void {
	let addr = VK_ORIG.gipa()(instance, name);
	if addr.is_null() { return addr; };
	match CStr::from_ptr(name).to_bytes() {
		b"vkCreateDevice" => {
			trace!("GTINS: vkCreateDevice");
			VK_ORIG.cd.set(std::mem::transmute(addr)).ok();
			vk_cd_hook as *mut c_void
		}
		b"vkCreateInstance" => {
			trace!("GTINS: vkCreateInstance");
			VK_ORIG.ci.set(std::mem::transmute(addr)).ok();
			vk_ci_hook as *mut c_void
		}
		b"vkAcquireNextImageKHR" => {
			trace!("GTINS: vkAcquireNextImageKHR");
			VK_ORIG.ani.set(std::mem::transmute(addr)).ok();
			vk_ani_khr_hook as *mut c_void
		}
		b"vkCreateSwapchainKHR" => {
			trace!("GTINS: vkCreateSwapchainKHR");
			VK_ORIG.cs.set(std::mem::transmute(addr)).ok();
			vk_cs_khr_hook as *mut c_void
		}
		b"vkQueuePresentKHR" => {
			trace!("GTINS: vkQueuePresentKHR");
			VK_ORIG.qp.set(std::mem::transmute(addr)).ok();
			vk_qp_khr_hook as *mut c_void
		}
		b"vkGetDeviceProcAddr" => {
			trace!("GTINS: vkGetDeviceProcAddr");
			VK_ORIG.gdpa.set(std::mem::transmute(addr)).ok();
			vk_gdpa_hook as *mut c_void
		}
		_ => {addr}
	}
}

unsafe extern "C" fn vk_gdpa_hook(device: VkDevice, name: *const c_char) -> *mut c_void {
	let addr = VK_ORIG.gdpa()(device, name);
	if addr.is_null() { return addr; };
	match CStr::from_ptr(name).to_bytes() {
		b"vkCreateFramebuffer" => {
			trace!("GTDEV: vkCreateFramebuffer");
			VK_ORIG.cf.set(std::mem::transmute(addr)).ok();
			vk_cf_hook as *mut c_void
		}
		b"vkDestroyFramebuffer" => {
			trace!("GTDEV: vkDestroyFramebuffer");
			VK_ORIG.df.set(std::mem::transmute(addr)).ok();
			vk_df_hook as *mut c_void
		}
		b"vkGetDeviceQueue" => {
			trace!("GTDEV: vkGetDeviceQueue");
			VK_ORIG.gdq.set(std::mem::transmute(addr)).ok();
			vk_gdq_hook as *mut c_void
		}
		_ => {addr}
	}
}

unsafe extern "C" fn vk_ani_khr_hook(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, image_index: *mut u32) -> VkResult {
	trace!("[CALLED]: AcquireNextImageKHR");
	let ret = VK_ORIG.ani()(device, swapchain, timeout, semaphore, fence, image_index);
	if ret == VK_SUCCESS {
		let mut st = get_state_mut();
		st.img_index = *image_index;
		trace!("SYNC: Acquire Index {}", *image_index);
	}
	ret
}

unsafe extern "C" fn vk_cs_khr_hook(device: VkDevice, p_create_info: *const VkSwapchainCreateInfoKHR, allocator: GeneralPtr, p_swapchain: *mut VkSwapchainKHR) -> VkResult {
	trace!("[CALLED]: CreateSwapchainKHR");
	let ret = VK_ORIG.cs()(device, p_create_info, allocator, p_swapchain);
	if ret == VK_SUCCESS && !p_create_info.is_null() {
		let mut st = get_state_mut();
		st.screen_extent = (*p_create_info).image_extent;
		st.img_min = (*p_create_info).min_image_count;
		trace!("SYNC: {:?}", (*p_create_info).image_extent);
		st.framebuffers.clear();

		// Init functions ptr pool.
		let gdpa = VK_ORIG.gdpa();
		VK_API.ccp.set(std::mem::transmute(gdpa(device, c"vkCreateCommandPool".as_ptr()))).ok();
		VK_API.cdp.set(std::mem::transmute(gdpa(device, c"vkCreateDescriptorPool".as_ptr()))).ok();
		VK_API.qs.set( std::mem::transmute(gdpa(device, c"vkQueueSubmit".as_ptr()))).ok();
		VK_API.acb.set(std::mem::transmute(gdpa(device, c"vkAllocateCommandBuffers".as_ptr()))).ok();
		VK_API.bcb.set(std::mem::transmute(gdpa(device, c"vkBeginCommandBuffer".as_ptr()))).ok();
		VK_API.ecb.set(std::mem::transmute(gdpa(device, c"vkEndCommandBuffer".as_ptr()))).ok();
		VK_API.brp.set(std::mem::transmute(gdpa(device, c"vkCmdBeginRenderPass".as_ptr()))).ok();
		VK_API.erp.set(std::mem::transmute(gdpa(device, c"vkCmdEndRenderPass".as_ptr()))).ok();
		VK_API.cff.set(std::mem::transmute(gdpa(device, c"vkCreateFence".as_ptr()))).ok();
		VK_API.wff.set(std::mem::transmute(gdpa(device, c"vkWaitForFences".as_ptr()))).ok();
		VK_API.rff.set(std::mem::transmute(gdpa(device, c"vkResetFences".as_ptr()))).ok();
		trace!("FN Pool inited: {:?}", VK_API);
		
		// Init command and descriptor pool.
		if st.command_pool.is_null() {
			// Allocate command pool.
			let pool_info = VkCommandPoolCreateInfo {
				s_type: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
				p_next: ptr::null(),
				flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
				queue_family_index: st.queue_family,
			};
			VK_API.ccp()(device, &pool_info, GeneralPtr::null(), &mut st.command_pool);

			// Allocate command buffers.
			let alloc_info = VkCommandBufferAllocateInfo {
				s_type: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
				p_next: ptr::null(),
				command_pool: st.command_pool,
				level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
				command_buffer_count: 8,
			};
			VK_API.acb()(device, &alloc_info, st.cmd_bufs.as_mut_ptr());

			// Create fences.
			let fence_info = VkFenceCreateInfo {
				s_type: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
				p_next: ptr::null(),
				flags: VK_FENCE_CREATE_SIGNALED_BIT,
			};
			for f in st.fences.iter_mut() {
				VK_API.cff()(device, &fence_info, GeneralPtr::null(), f);
			}
		}

		if st.descriptor_pool.is_null() {
			// Allocate descriptor pool
			let pool_size = VkDescriptorPoolSize {
				type_: VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
				descriptor_count: 1000,
			};
			let pool_info = VkDescriptorPoolCreateInfo {
				s_type: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
				p_next: ptr::null(),
				flags: VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT,
				max_sets: 1000,
				pool_size_count: 1,
				p_pool_sizes: &pool_size,
			};
			VK_API.cdp()(device, &pool_info, GeneralPtr::null(), &mut st.descriptor_pool);
		}
	}
	ret
}

unsafe extern "C" fn vk_cd_hook(phys_dev: VkPhysicalDevice, p_create_info: GeneralPtr, allocator: GeneralPtr, p_device: *mut VkDevice) -> VkResult {
	trace!("[CALLED]: CreateDevice");
	let ret = VK_ORIG.cd()(phys_dev, p_create_info, allocator, p_device);
	if ret == VK_SUCCESS {
		let mut st = get_state_mut();
		st.phys_dev = phys_dev;
		st.dev = *p_device;
		trace!("CRTDEV: Device captured {:?}", *p_device);
	}
	ret
}

unsafe extern "C" fn vk_ci_hook(create_info: GeneralPtr, allocator: GeneralPtr, p_instance: *mut VkInstance) -> VkResult {
	trace!("[CALLED]: CreateInstance");
	let ret = VK_ORIG.ci()(create_info, allocator, p_instance);
	if ret == VK_SUCCESS {
		let mut st = get_state_mut();
		st.instance = *p_instance;
		trace!("CRTINS: Instance captured {:?}", *p_instance);
	}
	ret
}

unsafe extern "C" fn vk_cf_hook(device: VkDevice, p_create_info: *const VkFramebufferCreateInfo, allocator: GeneralPtr, p_framebuffer: *mut VkFramebuffer) -> VkResult {
	trace!("[CALLED]: CreateFramebuffer");
	let ret = VK_ORIG.cf()(device, p_create_info, allocator, p_framebuffer);
	if ret == VK_SUCCESS {
		let mut st = get_state_mut();
		if st.screen_extent.width > 0 &&
			(*p_create_info).width == st.screen_extent.width &&
			(*p_create_info).height == st.screen_extent.height {
			if let Some(pos) = st.framebuffers.iter().position(|&x| x.is_null()) {
				st.framebuffers[pos] = *p_framebuffer;
			} else {
				st.framebuffers.push(*p_framebuffer);
			}
			st.render_pass = (*p_create_info).render_pass;
		}
	}
	ret
}

unsafe extern "C" fn vk_df_hook(device: VkDevice, framebuffer: VkFramebuffer, allocator: GeneralPtr) -> () {
	trace!("[CALLED]: DestroyFramebuffer");
	let mut st = get_state_mut();
	if let Some(pos) = st.framebuffers.iter().position(|&x| x == framebuffer) {
		st.framebuffers[pos] = GeneralPtr::null();
	}
	VK_ORIG.df()(device, framebuffer, allocator);
}

unsafe extern "C" fn vk_gdq_hook(device: VkDevice, queue_family_idx: u32, queue_idx: u32, p_queue: *mut VkQueue) -> () {
	trace!("[CALLED]: GetDeviceQueue");
	VK_ORIG.gdq()(device, queue_family_idx, queue_idx, p_queue);
	let mut st = get_state_mut();
	st.queue = *p_queue;
	st.queue_family = queue_family_idx;
	trace!("GTDEVQ: Queue captured {:?}", *p_queue);
}

unsafe extern "C" fn vk_qp_khr_hook(queue: VkQueue, p_present_info: *const VkPresentInfoKHR) -> VkResult {
	trace!("[CALLED]: QueuePresentKHR");
	imgui_init();

	let (fb, cmd_buf, fence, dev, queue, rp, ext) = {
		let st = get_state();
		if !st.imgui_inited || (st.img_index as usize) >= st.framebuffers.len() {
			return VK_ORIG.qp()(queue, p_present_info);
		}
		let idx = (st.img_index as usize) % 8;
		let fb = st.framebuffers[st.img_index as usize];
		if fb.is_null() {
			return VK_ORIG.qp()(queue, p_present_info);
		}
		(fb, st.cmd_bufs[idx], st.fences[idx], st.dev, st.queue, st.render_pass, st.screen_extent)
	};

	render_frame(fb, cmd_buf, fence, dev, queue, rp, ext);

	VK_ORIG.qp()(queue, p_present_info)
}

unsafe fn render_frame(fb: VkFramebuffer, cmd_buf: VkCommandBuffer, fence: VkFence, dev: VkDevice, queue: VkQueue, rp: VkRenderPass, ext: VkExtent2D) {
	if fb.is_null() || cmd_buf.is_null() || fence.is_null() || dev.is_null() || queue.is_null() || rp.is_null() {
		error!("[Vulkan]: render_frame arg is nullptr!");
		return;
	}

	let timeout_ns: u64 = 1_000_000_000;
	if VK_API.wff()(dev, 1, &fence, 1, timeout_ns) == VK_TIMEOUT {
		warn!("[Vulkan]: Fence wait timeout!");
		return;
	}
	VK_API.rff()(dev, 1, &fence);

	let begin_info = VkCommandBufferBeginInfo {
		s_type: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
		p_next: ptr::null(),
		flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
		p_inheritance_info: ptr::null(),
	};
	VK_API.bcb()(cmd_buf, &begin_info);

	let render_pass_info = VkRenderPassBeginInfo {
		s_type: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
		p_next: ptr::null(),
		render_pass: rp,
		framebuffer: fb,
		render_area: VkRect2D {
			offset: VkOffset2D { x: 0, y: 0 },
			extent: ext,
		},
		clear_value_count: 0,
		p_clear_values: ptr::null(),
	};
	VK_API.brp()(cmd_buf, &render_pass_info, VK_SUBPASS_CONTENTS_INLINE);

	imgui::ImGui_ImplVulkan_NewFrame();
	imgui::update_delta_time();
	imgui::igNewFrame();

	crate::menu::render_menu(true);

	imgui::igRender();
	imgui::ImGui_ImplVulkan_RenderDrawData(imgui::igGetDrawData(), cmd_buf.as_ptr(), 0);

	VK_API.erp()(cmd_buf);
	VK_API.ecb()(cmd_buf);

	let submit_info = VkSubmitInfo {
		s_type: VK_STRUCTURE_TYPE_SUBMIT_INFO,
		p_next: ptr::null(),
		wait_semaphore_count: 0,
		p_wait_semaphores: ptr::null(),
		p_wait_dst_stage_mask: ptr::null(),
		command_buffer_count: 1,
		p_command_buffers: &cmd_buf,
		signal_semaphore_count: 0,
		p_signal_semaphores: ptr::null(),
	};
	VK_API.qs()(queue, 1, &submit_info, fence);
}

unsafe fn imgui_init() {
	let st = get_state();
	if !st.imgui_inited {
		if st.instance.is_null() || st.render_pass.is_null() ||
			st.phys_dev.is_null() || st.dev.is_null() ||
			st.command_pool.is_null() || st.descriptor_pool.is_null() ||
			st.queue.is_null() || st.framebuffers.len() < st.img_min as usize {
			return;
		}

		imgui::init_context(
			st.screen_extent.width as f32,
			st.screen_extent.height as f32,
		);

		let mut init_info: imgui::ImGui_ImplVulkan_InitInfo = std::mem::zeroed();

		init_info.api_version = VK_API_VERSION_1_0;
		init_info.instance = st.instance;
		init_info.phys_dev = st.phys_dev;
		init_info.device = st.dev;
		init_info.queue_family = st.queue_family;
		init_info.queue = st.queue;
		init_info.descriptor_pool = st.descriptor_pool;
		init_info.min_img_count = st.img_min;
		init_info.img_count = st.framebuffers.len() as u32;
		init_info.min_alloc_size = 1024 * 1024;

		init_info.pipeline_info_main.render_pass = st.render_pass;
		init_info.pipeline_info_main.msaa_samples = VK_SAMPLE_COUNT_1_BIT;

		// Drop state reference before calling LoadFunctions to prevent deadlock.
		// We can pass device and instance pointers via user_data.
		// Instead using `get_state()` in `vulkan_loader`. TODO ?
		drop(st);
		
		imgui::ImGui_ImplVulkan_LoadFunctions(VK_API_VERSION_1_0, vulkan_loader, ptr::null_mut());
		imgui::ImGui_ImplVulkan_Init(&mut init_info);

		get_state_mut().imgui_inited = true;
		trace!("[IMGUI]: Initialized Vulkan backend");
	}
}

unsafe extern "C" fn vulkan_loader(function_name: *const c_char, _user_data: *mut c_void) -> *mut c_void {
	let st = get_state();
	let mut ptr = VK_ORIG.gdpa()(st.dev, function_name);
	if ptr.is_null() {
		ptr = VK_ORIG.gipa()(st.instance, function_name);
	}
	ptr
}
