mod logger;
mod and64inlinehook;
mod imgui;
mod ui;
mod vulkan;
mod egl;
mod menu;
mod input;

#[macro_use] extern crate log;

use log::LevelFilter;
use crate::and64inlinehook::init_hook_pool;

#[cfg(not(all(target_arch = "aarch64", target_os = "android")))]
compile_error!("Only aarch64-android is supported");

fn lib_main() {
	crate::logger::init_with_level("android_imgui_menu", LevelFilter::Trace).unwrap();
	setup_panic_hook();

	trace!("this is printed by default");
	error!("this is printed by default");

	unsafe {
		init_hook_pool();
	}

	std::thread::spawn(input::init);
	std::thread::spawn(egl::init);
	std::thread::spawn(vulkan::init);
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

// Safety: Called by linker. idk what to write here. fuck rust guidelines
pub unsafe extern "C" fn __android_init() {
	std::thread::spawn(lib_main);
}

#[used]
#[unsafe(link_section = ".init_array")]
static __INIT_HOOK: unsafe extern "C" fn() = __android_init;
