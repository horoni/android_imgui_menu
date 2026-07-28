#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::{c_long, c_void};
use std::sync::OnceLock;
use std::ptr;

use crate::and64inlinehook::a64_hook_function;
use crate::xdl;
use crate::imgui;

type PfnInput1 = unsafe extern "C" fn(thiz: *mut c_void, ex_ab: *const c_void, ex_ac: *const c_void);
type PfnInput2 = unsafe extern "C" fn(consumer: *mut c_void, factory: *mut c_void, is_raw: bool, sequence_id: c_long, out_policy_flags: *mut u32, out_event_ptr: *mut *const c_void) -> i32;

static ORIG_INPUT1: OnceLock<PfnInput1> = OnceLock::new();
static ORIG_INPUT2: OnceLock<PfnInput2> = OnceLock::new();

unsafe extern "C" fn input1_hook(thiz: *mut c_void, ex_ab: *const c_void, ex_ac: *const c_void) {
	ORIG_INPUT1.get().unwrap_or_else(|| {
		panic!("ORIG_INPUT1: Uninitialized!")
	})(thiz, ex_ab, ex_ac);
	if imgui::igGetCurrentContext().is_null() {
		return;
	}
	imgui::ImGui_ImplAndroid_HandleInputEvent(thiz);
}

unsafe extern "C" fn input2_hook(consumer: *mut c_void, factory: *mut c_void, is_raw: bool, sequence_id: c_long, out_policy_flags: *mut u32, out_event_ptr: *mut *const c_void) -> i32 {
	let ret: i32 = ORIG_INPUT2.get().unwrap_or_else(|| {
		panic!("ORIG_INPUT2: Uninitialized!")
	})(consumer, factory, is_raw, sequence_id, out_policy_flags, out_event_ptr);
	if ret != 0 || out_event_ptr.is_null() || (*out_event_ptr).is_null() || imgui::igGetCurrentContext().is_null() {
		return ret;
	}

	imgui::ImGui_ImplAndroid_HandleInputEvent(*out_event_ptr);

	ret
}

pub fn init() {
	let Some(lib_input) = xdl::Xdl::open_poll("libinput.so", 0, 300) else {
		warn!("[Input]: libinput.so not found after 3 sec");
		return;
	};

	let input1_addr = lib_input.sym("_ZN7android13InputConsumer21initializeMotionEventEPNS_11MotionEventEPKNS_12InputMessageE", None).unwrap_or(ptr::null_mut());
	let input2_addr = lib_input.sym("_ZN7android13InputConsumer7consumeEPNS_26InputEventFactoryInterfaceEblPjPPNS_10InputEventE", None).unwrap_or(ptr::null_mut());

	unsafe {
		if let Some(tramp) = a64_hook_function(input1_addr.cast(), input1_hook as *const u32) {
			let _ = ORIG_INPUT1.set(std::mem::transmute(tramp));
		} else if let Some(tramp) = a64_hook_function(input2_addr.cast(), input2_hook as *const u32) {
			let _ = ORIG_INPUT2.set(std::mem::transmute(tramp));
		}
	}
}
