#![allow(unsafe_op_in_unsafe_fn)]

use crate::vulkan;
use crate::imgui;
use std::ffi::{c_int, c_double};
use std::ptr;

pub unsafe fn render_menu(is_vulkan: bool) {
	imgui::igBegin(c"Menu".as_ptr(), ptr::null_mut(), 0);
	imgui::igText(c"Hello from ImGui!".as_ptr());
	imgui::igText(c"Vulkan?: %d".as_ptr(), is_vulkan as c_int);
	if is_vulkan {
		let st = vulkan::get_state();
		imgui::igText(c"image index: %d".as_ptr(), st.img_index);
		imgui::igText(c"framebuffers: %d".as_ptr(), st.framebuffers.len());
	}
	imgui::igText(c"delta: %f".as_ptr(), (*imgui::igGetIO_Nil()).delta_time as c_double);
	imgui::igEnd();
}
