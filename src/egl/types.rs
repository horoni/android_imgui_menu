use std::ffi::c_void;

pub type EGLDisplay = *mut c_void;
pub type EGLSurface = *mut c_void;
pub type EGLBoolean = i32;
pub type EGLint = i32;

pub const EGL_WIDTH: EGLint = 0x3057;
pub const EGL_HEIGHT: EGLint = 0x3056;

pub const GL_FRAMEBUFFER_SRGB: u32 = 0x8DB9;

pub type PfnEglSwapBuffers = unsafe extern "C" fn(dpy: EGLDisplay, surf: EGLSurface) -> EGLBoolean;
pub type PfnEglQuerySurface = unsafe extern "C" fn(dpy: EGLDisplay, surf: EGLSurface, attribute: EGLint, value: *mut EGLint) -> EGLBoolean;
pub type PfnGlIsEnabled = unsafe extern "C" fn(cap: u32) -> u8;
pub type PfnGlEnable = unsafe extern "C" fn(cap: u32);
pub type PfnGlDisable = unsafe extern "C" fn(cap: u32);

// Use GetProcAddr instead dlsym?
//pub type PfnEglGetProcAddr = unsafe extern "C" fn(procname: *const c_char) -> *mut c_void;
