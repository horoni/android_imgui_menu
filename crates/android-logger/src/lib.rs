#![allow(dead_code)]

use log::{Log, Level, LevelFilter, Metadata, Record, set_logger, set_max_level};
use std::ffi::{c_int, c_char};
use std::sync::OnceLock;
use std::io::Write;

unsafe extern "C" {
	fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

static LOGGER: OnceLock<AndroidLogger> = OnceLock::new();

struct AndroidLogger {
	tag: &'static str,
}

impl AndroidLogger {
	fn new(tag: String) -> Self {
		let tag = if tag.ends_with('\0') { tag } else { format!("{tag}\0") };
		let tag: &'static str = Box::leak(tag.into_boxed_str());
		Self { tag }
	}
}

impl Log for AndroidLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= log::max_level()
	}

	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let priority = match record.level() {
			Level::Error => 6,
			Level::Warn  => 5,
			Level::Info  => 4,
			Level::Debug => 3,
			Level::Trace => 2,
		};

		let mut buf = [0u8; 512];
		let heap: String;

		let msg_ptr = {
			let mut cursor = std::io::Cursor::new(&mut buf[..511]);
			if write!(cursor, "{}", record.args()).is_ok() {
				let pos = cursor.position() as usize;
				buf[pos] = 0;
				buf.as_ptr()
			} else {
				heap = format!("{}\0", record.args());
				heap.as_ptr()
			}
		};
		
		// Safety: tag and msg must be sentinel strings.
		unsafe {
			__android_log_write(
				priority,
				self.tag.as_ptr(),
				msg_ptr,
			);
		}
	}

	fn flush(&self) {}
}

/// Initialize logger with custom tag
/// # Safety
/// - Must be called only once before any log!() macros
/// - Tag must be valid utf8
/// # Errors
/// Returns `log::SetLoggerError` if logger already initialized
pub fn init(tag: impl Into<String>) -> Result<(), log::SetLoggerError> {
	let logger = LOGGER.get_or_init(|| AndroidLogger::new(tag.into()));
	set_logger(logger).map(|()| set_max_level(LevelFilter::Info))
}

/// Initialize logger with custom tag and level filter
pub fn init_with_level(tag: impl Into<String>, level: LevelFilter) -> Result<(), log::SetLoggerError> {
	let logger = LOGGER.get_or_init(|| AndroidLogger::new(tag.into()));
	set_logger(logger).map(|()| set_max_level(level))
}

/// Check if logger is already initialized
pub fn is_initialized() -> bool {
	LOGGER.get().is_some()
}
