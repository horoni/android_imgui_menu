#![allow(unused)]

use crate::imgui::*;
use super::cstr::IntoCStr;

pub struct TableToken;

impl TableToken {
	pub fn setup_column(&self, label: impl IntoCStr) {
		unsafe {
			igTableSetupColumn(label.as_c_ptr(), 0, 0.0, 0);
		}
	}

	pub fn headers_row(&self) {
		unsafe {
			igTableHeadersRow();
		}
	}

	pub fn next_row(&self) {
		unsafe {
			igTableNextRow(0, 0.0);
		}
	}

	pub fn next_column(&self) -> bool {
		unsafe { igTableNextColumn() }
	}

	pub fn cell(&self, f: impl FnOnce()) {
		if self.next_column() {
			f();
		}
	}
}
