#![allow(unsafe_op_in_unsafe_fn)]

use std::{ffi::{c_char, c_int, c_void}, ptr};
use crate::vulkan::types::*;
use std::time::Instant;

pub type PfnImGuiVulkanLoader = unsafe extern "C" fn(function_name: *const c_char, user_data: *mut c_void) -> *mut c_void;

#[repr(C)]
pub struct ImGuiContext;

#[repr(C)]
pub struct ImGuiStyle {
	pub font_size_base: f32,
	pub font_scale_main: f32,
	pub font_scale_dpi: f32,
	// ... Not a full struct
}

#[repr(C)]
pub struct ImGuiIO {
	pub config_flags: i32,
	pub backend_flags: i32,
	pub display_size: ImVec2,
	pub display_framebuffer_scale: ImVec2,
	pub delta_time: f32,
	pub ini_saving_rate: f32,
	pub ini_filename: *const c_char,
	pub log_filename: *const c_char,
	// ... Not a full struct
}

unsafe extern "C" {
	pub fn igCreateContext(shared_font_atlas: *mut c_void) -> *mut ImGuiContext;
	pub fn igGetCurrentContext() -> *mut ImGuiContext;
	pub fn igNewFrame();
	pub fn igRender();
	pub fn igGetDrawData() -> *mut c_void;
	pub fn igGetIO_Nil() -> *mut ImGuiIO;
	pub fn igGetStyle() -> *mut ImGuiStyle;
	pub fn ImGuiStyle_ScaleAllSizes(self_: *mut ImGuiStyle, scale_factor: f32);

	pub fn igBegin(name: *const c_char, p_open: *mut bool, flags: i32) -> bool;
	pub fn igText(fmt: *const c_char, ...);
	pub fn igEnd();

	pub fn ImGui_ImplVulkan_LoadFunctions(api_version: u32, loader_func: PfnImGuiVulkanLoader, user_data: *mut c_void) -> bool;
	pub fn ImGui_ImplVulkan_Init(init_info: *mut ImGui_ImplVulkan_InitInfo) -> bool;
	pub fn ImGui_ImplVulkan_NewFrame();
	pub fn ImGui_ImplVulkan_RenderDrawData(draw_data: *mut c_void, command_buffer: *mut c_void, pipeline: u64);

	pub fn ImGui_ImplOpenGL3_Init(glsl_version: *const c_char) -> bool;
	pub fn ImGui_ImplOpenGL3_NewFrame();
	pub fn ImGui_ImplOpenGL3_RenderDrawData(draw_data: *mut c_void);

	pub fn ImGui_ImplAndroid_HandleInputEvent(input_event: *const c_void) -> i32;
}

#[repr(C)]
pub struct ImGui_ImplVulkan_InitInfo {
	pub api_version: u32,
	pub instance: VkInstance,
	pub phys_dev: VkPhysicalDevice,
	pub device: VkDevice,
	pub queue_family: u32,
	pub queue: VkQueue,
	pub descriptor_pool: VkDescriptorPool,
	pub descriptor_pool_size: u32,
	pub min_img_count: u32,
	pub img_count: u32,
	pub pipeline_cache: GeneralPtr,
	pub pipeline_info_main: ImGui_ImplVulkan_PipelineInfo,
	pub pipeline_info_viewport: ImGui_ImplVulkan_PipelineInfo,
	pub use_dynamic_rendering: bool,
	pub allocator: GeneralPtr,
	pub check_vk_result_fn: GeneralPtr,
	pub min_alloc_size: u64,
	pub custom_shader_vert_create_info: VkShaderModuleCreateInfo,
	pub custom_shader_frag_create_info: VkShaderModuleCreateInfo,
}

#[repr(C)]
pub struct ImGui_ImplVulkan_PipelineInfo {
	pub render_pass: VkRenderPass,
	pub subpass: u32,
	pub msaa_samples: i32,
	pub extra_dynamic_states: ImVector_VkDynamicState,
	pub pipeline_rendering_create_info: VkPipelineRenderingCreateInfoKHR,
	pub swap_chain_image_usage: VkImageUsageFlags,
}

#[repr(C)]
pub struct ImVector_VkDynamicState {
	pub size: c_int,
	pub capacity: c_int,
	pub data: *mut VkDynamicState,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ImVec2 {
	pub x: f32,
	pub y: f32,
}

/// # Use
/// Must be called right before `igNewFrame()`
pub unsafe fn update_delta_time() {
	static mut LAST_TIME: Option<Instant> = None;

	let now = Instant::now();
	let dt = match LAST_TIME {
		Some(last) => now.duration_since(last).as_secs_f32(),
		None => 1.0 / 60.0,
	};
	LAST_TIME = Some(now);

	let io = igGetIO_Nil();
	(*io).delta_time = if dt > 0.00001 { dt } else { 0.016667 };
}

/// Initializes ImGui context
/// If context is already initialized, sets display size with `w`, `h`
pub unsafe fn init_context(w: f32, h: f32) {
	if !igGetCurrentContext().is_null() {
		let io = igGetIO_Nil();
		(*io).display_size = ImVec2 { x: w, y: h };
		return;
	}

	igCreateContext(std::ptr::null_mut());

	let style = igGetStyle();
	ImGuiStyle_ScaleAllSizes(style, 2.0);
	(*style).font_scale_dpi = 2.0;

	let io = igGetIO_Nil();
	(*io).display_size = ImVec2 { x: w, y: h };
	(*io).ini_filename = ptr::null();
}
