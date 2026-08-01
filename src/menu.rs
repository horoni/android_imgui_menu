#![allow(unsafe_op_in_unsafe_fn)]

use crate::vulkan;
use imgui_rs::ffi as imgui;
use imgui_rs::{ui, text, fmt_c};
use std::cell::RefCell;

pub fn render_menu(is_vulkan: bool) {
	static mut A: bool = false;
	thread_local! {
		static VULKAN_FB_TABLE: RefCell<bool> = RefCell::new(false);
		static COUNTER: RefCell<u32> = RefCell::new(0);
	}

	let ui = ui::Ui::new();
	ui.window(c"Menu")
		.size(300.0, 400.0, imgui::ImGuiCond_Once)
		.build(|| {
		ui.text(c"Hello from ImGui");
		ui.checkbox(c"Some checkbox", &raw mut A);
		ui.text(fmt_c!("Vulkan?: {}", is_vulkan));
		if is_vulkan {
			let st = vulkan::get_state();
			ui.text(fmt_c!("image index: {}", st.img_index));
			ui.text(fmt_c!("framebuffers: {}", st.framebuffers.len()));
			VULKAN_FB_TABLE.with_borrow_mut(|v| {
				ui.checkbox(c"FB table", v);
				if *v {
					ui.table(c"framebuffers_table", 2)
						.sizing_fixed_fit()
						.build(|t| {
						t.setup_column(c"idx");
						t.setup_column(c"ptr");
						t.headers_row();

						let mut idx = 0usize;
						for fb in &st.framebuffers {
							t.next_row();
							t.cell(|| ui.text(fmt_c!("{}", idx)));
							t.cell(|| ui.text(fmt_c!("{:?}", fb)));
							idx += 1;
						}
					});
				}
			});
		}
		COUNTER.with(|v| {
			let mut a = v.borrow_mut();
			ui.text(fmt_c!("Counter: {}", a));
			if ui.button("Click") {
				*a += 1;
			}
		});
		let delta = unsafe {
			(*imgui::igGetIO_Nil()).delta_time
		};
		text!(ui, "delta: {}", delta);
		text!(ui, "fps: {}", 1.0 / delta);
	});
}
