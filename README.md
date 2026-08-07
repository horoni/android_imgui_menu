# android_imgui_menu
## How to use
add build.rs with following contents:
```Rust
fn main() {
	// For ImGui
	println!("cargo:rustc-link-lib=static=c++_static");
	// For EGL backend
	println!("cargo:rustc-link-lib=GLESv3");
	println!("cargo:rustc-link-lib=EGL");
	// For logs
	println!("cargo:rustc-link-lib=log");
	// For ImGui android backend
	println!("cargo:rustc-link-lib=android");
}
```
add dependency to Cargo.toml:
```
# ...
[dependencies]
imgui-rs = { git = "https://github.com/horoni/android_imgui_menu" }
imgui-egl-hook = { git = "https://github.com/horoni/android_imgui_menu" }
imgui-vk-hook = { git = "https://github.com/horoni/android_imgui_menu" }
imgui-input-universal-hook = { git = "https://github.com/horoni/android_imgui_menu" }
android-logger = { git = "https://github.com/horoni/android_imgui_menu"}
and64inlinehook-rs = { git = "https://github.com/horoni/and64inlinehook-rs" }
```
do not forget to set crate type to dynamic library
```
[lib]
crate-type = ["cdylib"]
```
## Example
here is example lib.rs:
```
mod menu;

#[macro_use] extern crate log;

use log::LevelFilter;
use android_logger;
use and64inlinehook_rs::init_hook_pool;
use imgui_rs::PfnImGuiRender;
use std::sync::Arc;

#[cfg(not(all(target_arch = "aarch64", target_os = "android")))]
compile_error!("Only aarch64-android is supported");

fn lib_main() {
	android_logger::init_with_level("android_imgui_menu", LevelFilter::Trace).unwrap();
	setup_panic_hook();

	trace!("this is printed by default");
	error!("this is printed by default");

	// Initialize hook pool for and64inlinehook. THIS IS MANDATORY!
	unsafe {
		init_hook_pool();
	}

	let render_fn: Arc<PfnImGuiRender> = Arc::new(|is_vulkan| {
		menu::render_menu(is_vulkan);
	});
	// Clone of render_fn ptr, because thread want to borrow it
	let render_fn2 = render_fn.clone();

	std::thread::spawn(imgui_input_universal_hook::init);
	std::thread::spawn(move || imgui_egl_hook::init(render_fn2));
	std::thread::spawn(move || imgui_vk_hook::init(render_fn));
}

fn setup_panic_hook() {
	std::panic::set_hook(Box::new(|info| {
		let (file, line) = info.location().map_or(("???", 0), |loc| (loc.file(), loc.line()));
		let msg = match info.payload().downcast_ref::<&str>() {
			Some(s) => *s,
			None => match info.payload().downcast_ref::<String>() {
				Some(s) => &s[..],
				None => "Box<Any>",
			},
		};

		error!("PANIC [{file}:{line}]: {msg}");

		// panic = "abort" in release
		std::process::abort();
	}));
}

// Safety: Called once by linker.
pub unsafe extern "C" fn __android_init() {
	std::thread::spawn(lib_main);
}

#[used]
#[unsafe(link_section = ".init_array")]
static __INIT_HOOK: unsafe extern "C" fn() = __android_init;
```
and example menu.rs
```
#![allow(unsafe_op_in_unsafe_fn)]

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
		ui.text(c"Hello from ImGui...");
		ui.checkbox(c"Some checkbox", &raw mut A);
		ui.text(fmt_c!("Vulkan?: {}", is_vulkan));
		if is_vulkan {
			let st = imgui_vk_hook::get_state();
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
```
