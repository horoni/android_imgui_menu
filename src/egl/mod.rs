#![allow(unsafe_op_in_unsafe_fn)]

mod types;

use crate::and64inlinehook::a64_hook_function;
use crate::egl::types::*;
use crate::imgui;
use crate::xdl;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::ptr;

static ORIG_SWAPBUFFERS: OnceLock<PfnEglSwapBuffers> = OnceLock::new();
static EGL_QUERY_SURFACE: OnceLock<PfnEglQuerySurface> = OnceLock::new();
static GL_IS_ENABLED: OnceLock<PfnGlIsEnabled> = OnceLock::new();
static GL_ENABLE: OnceLock<PfnGlEnable> = OnceLock::new();
static GL_DISABLE: OnceLock<PfnGlDisable> = OnceLock::new();

static IMGUI_INITED: AtomicBool = AtomicBool::new(false);

pub fn init() {
	let Some(lib_egl) = xdl::Xdl::open_poll("libEGL.so", 0, 300) else {
		warn!("[EGL]: libEGL.so not found after 3 sec");
		return;
	};
	let Some(lib_gles) = xdl::Xdl::open_poll("libGLESv3.so", 0, 300) else {
		warn!("[EGL]: libGLESv3.so not found after 3 sec");
		return;
	};

	let swapbuffers_addr = lib_egl.sym("eglSwapBuffers", None).unwrap_or(ptr::null_mut());
	let query_addr = lib_egl.sym("eglQuerySurface", None);
	let gl_is_enabled_addr = lib_gles.sym("glIsEnabled", None);
	let gl_enable_addr = lib_gles.sym("glEnable", None);
	let gl_disable_addr = lib_gles.sym("glDisable", None);

	unsafe {
		if let Some(tramp) = a64_hook_function(swapbuffers_addr.cast(), egl_swapbuffers_hook as *const u32) {
			let _ = ORIG_SWAPBUFFERS.set(std::mem::transmute(tramp));
		}
		if let Some(addr) = query_addr {
			let _ = EGL_QUERY_SURFACE.set(std::mem::transmute(addr));
		}
		if let Some(addr) = gl_is_enabled_addr {
			let _ = GL_IS_ENABLED.set(std::mem::transmute(addr));
		}
		if let Some(addr) = gl_enable_addr {
			let _ = GL_ENABLE.set(std::mem::transmute(addr));
		}
		if let Some(addr) = gl_disable_addr {
			let _ = GL_DISABLE.set(std::mem::transmute(addr));
		}
	}
}

unsafe extern "C" fn egl_swapbuffers_hook(dpy: EGLDisplay, surf: EGLSurface) -> EGLBoolean {
	let mut width: EGLint = 0;
	let mut height: EGLint = 0;

	let query = EGL_QUERY_SURFACE.get().unwrap();
	query(dpy, surf, EGL_WIDTH, &mut width);
	query(dpy, surf, EGL_HEIGHT, &mut height);

	imgui::init_context(width as f32, height as f32);
	init_imgui();

	imgui::ImGui_ImplOpenGL3_NewFrame();
	imgui::update_delta_time();
	imgui::igNewFrame();

	crate::menu::render_menu(false);

	imgui::igRender();

	let srgb = GL_IS_ENABLED.get().unwrap()(GL_FRAMEBUFFER_SRGB);
	if srgb != 0 {
		GL_DISABLE.get().unwrap()(GL_FRAMEBUFFER_SRGB);
	}
	imgui::ImGui_ImplOpenGL3_RenderDrawData(imgui::igGetDrawData());
	if srgb != 0 {
		GL_ENABLE.get().unwrap()(GL_FRAMEBUFFER_SRGB);
	}

	ORIG_SWAPBUFFERS.get().unwrap()(dpy, surf)
}

unsafe fn init_imgui() {
	if !IMGUI_INITED.swap(true, Ordering::Relaxed) {
		imgui::ImGui_ImplOpenGL3_Init(c"#version 300 es".as_ptr());
		trace!("[IMGUI]: Initialized EGL backend");
	}
}
