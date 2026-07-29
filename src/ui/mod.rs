#![allow(unused)]

pub mod cstr;
pub mod table;

use cstr::IntoCStr;
use table::TableToken;
use crate::imgui::*;

pub struct WindowBuilder {
	title: *const std::ffi::c_char,
	size: Option<(ImVec2, i32)>,
}

impl WindowBuilder {
	pub fn size(mut self, w: f32, h: f32, cond: i32) -> Self {
		self.size = Some((ImVec2 { x: w, y: h }, cond));
		self
	}

	pub fn build(self, f: impl FnOnce()) {
		unsafe {
			if let Some((size, cond)) = self.size {
				igSetNextWindowSize(size, cond);
			}
			if igBegin(self.title, std::ptr::null_mut(), 0) {
				f();
			}
			igEnd();
		}
	}
}

pub struct TableBuilder {
	id: *const std::ffi::c_char,
	columns: i32,
	flags: i32,
	outer_size: ImVec2,
}

impl TableBuilder {
	pub fn flags(mut self, flags: i32) -> Self {
		self.flags |= flags;
		self
	}

	pub fn sizing_fixed_fit(self) -> Self {
		self.flags(ImGuiTableFlags_SizingFixedFit)
	}

	pub fn outer_size(mut self, size: ImVec2) -> Self {
		self.outer_size = size;
		self
	}

	pub fn build(self, f: impl FnOnce(&TableToken)) {
		unsafe {
			if igBeginTable(self.id, self.columns, self.flags, self.outer_size, 0.0) {
				let token = TableToken;
				f(&token);
				igEndTable();
			}
		}
	}
}

pub struct Ui;

impl Ui {
	pub fn new() -> Self {
		Self
	}

	pub fn window(&self, title: impl IntoCStr) -> WindowBuilder {
		WindowBuilder {
			title: title.as_c_ptr(),
			size: None,
		}
	}

	pub fn table(&self, id: impl IntoCStr, columns: i32) -> TableBuilder {
		TableBuilder {
			id: id.as_c_ptr(),
			columns,
			flags: 0,
			outer_size: ImVec2 { x: 0.0, y: 0.0 },
		}
	}

	pub fn button(&self, label: impl IntoCStr) -> bool {
		unsafe { igButton(label.as_c_ptr(), ImVec2 { x: 0.0, y: 0.0 }) }
	}

	pub fn checkbox(&self, label: impl IntoCStr, value: *mut bool) -> bool {
		unsafe { igCheckbox(label.as_c_ptr(), value) }
	}

	pub fn text(&self, label: impl IntoCStr) {
		unsafe { igText(label.as_c_ptr()) }
	}

	#[inline(always)]
	pub fn text_fmt(&self, args: std::fmt::Arguments) {
		let ptr = cstr::fmt_thr_stack(args);
		unsafe { igText(ptr) }
	}
}

#[macro_export]
macro_rules! text {
	($ui:expr, $msg:expr) => {
		$ui.text($msg)
	};
	($ui:expr, $fmt:literal, $($arg:tt)*) => {
		$ui.text_fmt(format_args!($fmt, $($arg)*))
	};
}
