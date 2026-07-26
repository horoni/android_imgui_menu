#![allow(unused)]

use std::ffi::{c_void, c_char};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneralPtr (*mut c_void);
unsafe impl Send for GeneralPtr {}
unsafe impl Sync for GeneralPtr {}
impl GeneralPtr {
	pub const fn null() -> Self {
		GeneralPtr(ptr::null_mut())
	}
	pub fn is_null(&self) -> bool {
		self.0.is_null()
	}
	pub fn as_ptr(&self) -> *mut c_void {
		self.0
	}
}

pub type VkInstance = GeneralPtr;
pub type VkPhysicalDevice = GeneralPtr;
pub type VkDevice = GeneralPtr;
pub type VkQueue = GeneralPtr;
pub type VkFence = GeneralPtr;
pub type VkSemaphore = GeneralPtr;
pub type VkCommandBuffer = GeneralPtr;
pub type VkRenderPass = GeneralPtr;
pub type VkFramebuffer = GeneralPtr;
pub type VkImageView = GeneralPtr;
pub type VkCommandPool = GeneralPtr;
pub type VkDescriptorPool = GeneralPtr;

pub type VkSubpassContents = i32; // enum
pub type VkDynamicState = i32; // enum
pub type VkShaderModuleCreateFlags = u32;
pub type VkCommandPoolCreateFlags = u32;
pub type VkDescriptorPoolCreateFlags = u32;
pub type VkDescriptorType = i32;
pub type VkCommandBufferLevel = i32;
pub type VkCommandBufferUsageFlags = u32;
pub type VkFenceCreateFlags = u32;
pub type VkQueryControlFlags = u32;
pub type VkQueryPipelineStatisticFlags = u32;
pub type VkFramebufferCreateFlags = u32;
pub type VkFormat = i32; // enum
pub type VkImageUsageFlags = u32;
pub type VkSharingMode = i32; // enum
pub type VkBool32 = u32;
pub type VkPipelineStageFlags = u32;
pub type VkResult = i32; // enum

pub const VK_API_VERSION_1_0: u32 = 1000000;

pub const VK_SUCCESS: VkResult = 0;
pub const VK_NOT_READY: VkResult = 1;
pub const VK_TIMEOUT: VkResult = 2;

pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: i32 = 4;
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: i32 = 8;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: i32 = 33;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: i32 = 39;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: i32 = 40;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: i32 = 42;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: i32 = 43;

pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: i32 = 0;
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: u32 = 0x00000001;

pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: u32 = 0x00000002;

pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: i32 = 1;
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: u32 = 0x00000001;

pub const VK_FENCE_CREATE_SIGNALED_BIT: u32 = 0x00000001;

pub const VK_SUBPASS_CONTENTS_INLINE: i32 = 0;

pub const VK_SAMPLE_COUNT_1_BIT: i32 = 0x00000001;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExtent2D {
	pub width: u32,
	pub height: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOffset2D {
	pub x: i32,
	pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRect2D {
	pub offset: VkOffset2D,
	pub extent: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassBeginInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO
	pub p_next: *const c_void,
	pub render_pass: VkRenderPass,
	pub framebuffer: VkFramebuffer,
	pub render_area: VkRect2D,
	pub clear_value_count: u32,
	pub p_clear_values: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFramebufferCreateInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO
	pub p_next: *const c_void,
	pub flags: VkFramebufferCreateFlags,
	pub render_pass: VkRenderPass,
	pub attachment_count: u32,
	pub p_attachments: *const VkImageView,
	pub width: u32,
	pub height: u32,
	pub layers: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct VkSubmitInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_SUBMIT_INFO
	pub p_next: *const c_void,
	pub wait_semaphore_count: u32,
	pub p_wait_semaphores: *const VkSemaphore,
	pub p_wait_dst_stage_mask: *const VkPipelineStageFlags,
	pub command_buffer_count: u32,
	pub p_command_buffers: *const VkCommandBuffer,
	pub signal_semaphore_count: u32,
	pub p_signal_semaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolCreateInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO
	pub p_next: *const c_void,
	pub flags: VkCommandPoolCreateFlags,
	pub queue_family_index: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorPoolSize {
	pub type_: VkDescriptorType,
	pub descriptor_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorPoolCreateInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO
	pub p_next: *const c_void,
	pub flags: VkDescriptorPoolCreateFlags,
	pub max_sets: u32,
	pub pool_size_count: u32,
	pub p_pool_sizes: *const VkDescriptorPoolSize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferAllocateInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO
	pub p_next: *const c_void,
	pub command_pool: VkCommandPool,
	pub level: VkCommandBufferLevel,
	pub command_buffer_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFenceCreateInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_FENCE_CREATE_INFO
	pub p_next: *const c_void,
	pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferInheritanceInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO
	pub p_next: *const c_void,
	pub render_pass: VkRenderPass,
	pub subpass: u32,
	pub framebuffer: VkFramebuffer,
	pub occlusion_query_enable: VkBool32,
	pub query_flags: VkQueryControlFlags,
	pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferBeginInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO
	pub p_next: *const c_void,
	pub flags: VkCommandBufferUsageFlags,
	pub p_inheritance_info: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShaderModuleCreateInfo {
	pub s_type: i32, // VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO
	pub p_next: *const c_void,
	pub flags: VkShaderModuleCreateFlags,
	pub code_size: usize,
	pub p_code: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRenderingCreateInfo {
	pub s_type: i32,
	pub p_next: *const c_void,
	pub view_mask: u32,
	pub color_attachment_count: u32,
	pub p_color_attachment_formats: *const VkFormat,
	pub depth_attachment_format: VkFormat,
	pub stencil_attachment_format: VkFormat,
}

pub type PfnVkQueueSubmit = unsafe extern "C" fn(queue: VkQueue, submit_count: u32, p_submits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
pub type PfnVkQueueWaitIdle = unsafe extern "C" fn(queue: VkQueue) -> VkResult;
pub type PfnVkGetDeviceProcAddr = unsafe extern "C" fn(device: VkDevice, name: *const c_char) -> *mut c_void;
pub type PfnVkGetInstanceProcAddr = unsafe extern "C" fn(instance: VkInstance, name: *const c_char) -> *mut c_void;
pub type PfnVkCreateFramebuffer = unsafe extern "C" fn(device: VkDevice, create_info: *const VkFramebufferCreateInfo, allocator: GeneralPtr, framebuffer: *mut VkFramebuffer) -> VkResult;
pub type PfnVkDestroyFramebuffer = unsafe extern "C" fn(device: VkDevice, framebuffer: VkFramebuffer, allocator: GeneralPtr);
pub type PfnVkCreateDevice = unsafe extern "C" fn(phys_dev: VkPhysicalDevice, create_info: GeneralPtr, allocator: GeneralPtr, p_device: *mut VkDevice) -> VkResult;
pub type PfnVkCreateInstance = unsafe extern "C" fn(create_info: GeneralPtr, allocator: GeneralPtr, p_instance: *mut VkInstance) -> VkResult;
pub type PfnVkGetDeviceQueue = unsafe extern "C" fn(device: VkDevice, queue_family_idx: u32, queue_idx: u32, p_queue: *mut VkQueue);
pub type PfnVkCreateCommandPool = unsafe extern "C" fn(device: VkDevice, p_create_info: *const VkCommandPoolCreateInfo, allocator: GeneralPtr, p_command_pool: *mut VkCommandPool) -> VkResult;
pub type PfnVkCreateDescriptorPool = unsafe extern "C" fn(device: VkDevice, p_create_info: *const VkDescriptorPoolCreateInfo, allocator: GeneralPtr, p_descriptor_pool: *mut VkDescriptorPool) -> VkResult;
pub type PfnVkAllocateCommandBuffers = unsafe extern "C" fn(device: VkDevice, p_allocate_info: *const VkCommandBufferAllocateInfo, p_command_buffers: *mut VkCommandBuffer) -> VkResult;
pub type PfnVkFreeCommandBuffers = unsafe extern "C" fn(device: VkDevice, command_pool: VkCommandPool, command_buffer_count: u32, p_command_buffers: *const VkCommandBuffer);
pub type PfnVkBeginCommandBuffer = unsafe extern "C" fn(command_buffer: VkCommandBuffer, p_begin_info: *const VkCommandBufferBeginInfo) -> VkResult;
pub type PfnVkEndCommandBuffer = unsafe extern "C" fn(command_buffer: VkCommandBuffer) -> VkResult;
pub type PfnVkCmdBeginRenderPass = unsafe extern "C" fn(command_buffer: VkCommandBuffer, p_render_pass_begin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
pub type PfnVkCmdEndRenderPass = unsafe extern "C" fn(command_buffer: VkCommandBuffer);
pub type PfnVkCreateFence = unsafe extern "C" fn(device: VkDevice, p_create_info: *const VkFenceCreateInfo, allocator: GeneralPtr, p_fence: *mut VkFence) -> VkResult;
pub type PfnVkWaitForFences = unsafe extern "C" fn(device: VkDevice, fence_count: u32, p_fences: *const VkFence, wait_all: VkBool32, timeout: u64) -> VkResult;
pub type PfnVkResetFences = unsafe extern "C" fn(device: VkDevice, fence_count: u32, p_fences: *const VkFence) -> VkResult;

// KHR
pub type VkSwapchainCreateFlagsKHR = u32;
pub type VkSurfaceKHR = GeneralPtr;
pub type VkColorSpaceKHR = i32; // enum
pub type VkSurfaceTransformFlagBitsKHR = i32; // enum
pub type VkCompositeAlphaFlagBitsKHR = i32; // enum
pub type VkPresentModeKHR = i32; // enum
pub type VkSwapchainKHR = GeneralPtr;

pub type VkPipelineRenderingCreateInfoKHR = VkPipelineRenderingCreateInfo;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSwapchainCreateInfoKHR {
	pub s_type: i32, // VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000
	pub p_next: *const c_void,
	pub flags: VkSwapchainCreateFlagsKHR,
	pub surface: VkSurfaceKHR,
	pub min_image_count: u32,
	pub image_format: VkFormat,
	pub image_color_space: VkColorSpaceKHR,
	pub image_extent: VkExtent2D,
	pub image_array_layers: u32,
	pub image_usage: VkImageUsageFlags,
	pub image_sharing_mode: VkSharingMode,
	pub queue_family_index_count: u32,
	pub p_queue_family_indices: *const u32,
	pub pre_transform: VkSurfaceTransformFlagBitsKHR,
	pub composite_alpha: VkCompositeAlphaFlagBitsKHR,
	pub present_mode: VkPresentModeKHR,
	pub clipped: VkBool32,
	pub old_swapchain: VkSwapchainKHR,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPresentInfoKHR {
	pub s_type: i32, // VK_STRUCTURE_TYPE_PRESENT_INFO_KHR = 1000001001
	pub p_next: *const c_void,
	pub wait_semaphore_count: u32,
	pub p_wait_semaphores: *const VkSemaphore,
	pub swapchain_count: u32,
	pub p_swapchains: *const VkSwapchainKHR,
	pub p_image_indices: *const u32,
	pub p_results: *mut VkResult,
}

pub type PfnVkAcquireNextImageKHR = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, image_index: *mut u32) -> VkResult;
pub type PfnVkCreateSwapchainKHR = unsafe extern "C" fn(device: VkDevice, p_create_info: *const VkSwapchainCreateInfoKHR, allocator: GeneralPtr, p_swapchain: *mut VkSwapchainKHR) -> VkResult;
pub type PfnVkQueuePresentKHR = unsafe extern "C" fn(queue: VkQueue, p_present_info: *const VkPresentInfoKHR) -> VkResult;
