#![allow(unsafe_op_in_unsafe_fn, unused, non_upper_case_globals)]

use std::{ffi::{c_char, c_float, c_int, c_void}, ptr};
use std::time::Instant;
use vulkan_rs::*;

pub type PfnImGuiVulkanLoader = unsafe extern "C" fn(function_name: *const c_char, user_data: *mut c_void) -> *mut c_void;

#[repr(C)]
pub struct ImGuiContext {
	// Get rid of warning about ffi safety
	_priv: [u8; 4],
}

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

pub const ImGuiCond_None: i32 = 0;
pub const ImGuiCond_Always: i32 = 1 << 0;
pub const ImGuiCond_Once: i32 = 1 << 1;
pub const ImGuiCond_FirstUseEver: i32 = 1 << 2;
pub const ImGuiCond_Appearing: i32 = 1 << 3;

pub const ImGuiTableFlags_None: i32 = 0;
pub const ImGuiTableFlags_Resizable: i32 = 1 << 0;
pub const ImGuiTableFlags_Reorderable: i32 = 1 << 1;
pub const ImGuiTableFlags_Hideable: i32 = 1 << 2;
pub const ImGuiTableFlags_Sortable: i32 = 1 << 3;
pub const ImGuiTableFlags_RowBg: i32 = 1 << 6;
pub const ImGuiTableFlags_BordersInnerH: i32 = 1 << 7;
pub const ImGuiTableFlags_BordersOuterH: i32 = 1 << 8;
pub const ImGuiTableFlags_BordersInnerV: i32 = 1 << 9;
pub const ImGuiTableFlags_BordersOuterV: i32 = 1 << 10;
pub const ImGuiTableFlags_BordersH: i32 = ImGuiTableFlags_BordersInnerH | ImGuiTableFlags_BordersOuterH;
pub const ImGuiTableFlags_BordersV: i32 = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersOuterV;
pub const ImGuiTableFlags_Borders: i32 = ImGuiTableFlags_BordersH | ImGuiTableFlags_BordersV;
pub const ImGuiTableFlags_SizingFixedFit: i32 = 1 << 13;
pub const ImGuiTableFlags_SizingFixedSame: i32 = 2 << 13;
pub const ImGuiTableFlags_SizingStretchProp: i32 = 3 << 13;
pub const ImGuiTableFlags_SizingStretchSame: i32 = 4 << 13;
pub const ImGuiTableFlags_ScrollX: i32 = 1 << 24;
pub const ImGuiTableFlags_ScrollY: i32 = 1 << 25;

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
	pub fn igEnd();
	pub fn igSetNextWindowSize(size: ImVec2, cond: i32);
	pub fn igText(fmt: *const c_char, ...);
	pub fn igCheckbox(label: *const c_char, v: *mut bool) -> bool;
	pub fn igButton(label: *const c_char, size: ImVec2) -> bool;
	pub fn igBeginTable(str_id: *const c_char, columns: c_int, flags: i32, outer_size: ImVec2, inner_width: c_float) -> bool;
	pub fn igEndTable();
	pub fn igTableNextRow(row_flags: i32, min_row_height: c_float);
	pub fn igTableNextColumn() -> bool;
	pub fn igTableSetupColumn(label: *const c_char, flags: i32, init_width_or_weight: c_float, user_id: i32);
	pub fn igTableHeadersRow();

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
