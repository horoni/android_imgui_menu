#![allow(unused)]

use std::cell::RefCell;
use std::ffi::{c_char, CStr};
use std::io::Write;

thread_local! {
	static STR_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(1024));
	static FMT_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(1024));
}

/// # Safety
/// Pointer only valid until next call on same thread.
pub trait IntoCStr {
	fn as_c_ptr(&self) -> *const c_char;
}

impl IntoCStr for &CStr {
	#[inline(always)]
	fn as_c_ptr(&self) -> *const c_char {
		self.as_ptr().cast()
	}
}

impl IntoCStr for &str {
	fn as_c_ptr(&self) -> *const c_char {
		STR_BUFFER.with(|buf| {
			let mut b = buf.borrow_mut();
			b.clear();
			b.extend_from_slice(self.as_bytes());
			b.push(0);
			b.as_ptr().cast()
		})
	}
}

impl IntoCStr for *const c_char {
	#[inline(always)]
	fn as_c_ptr(&self) -> *const c_char {
		*self
	}
}

pub fn fmt_thr_stack(args: std::fmt::Arguments) -> *const c_char {
	FMT_BUFFER.with(|buf| {
		let mut b = buf.borrow_mut();
		b.clear();
		write!(b, "{}", args);
		b.push(0);
		b.as_ptr().cast()
	})
}

#[macro_export]
macro_rules! fmt_c {
	($($arg:tt)*) => {
		$crate::ui::cstr::fmt_thr_stack(format_args!($($arg)*))
	};
}
