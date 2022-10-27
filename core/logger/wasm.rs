/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Arguments;

use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::{builder::Builder, echo::Echo, FilterFn, FormatFn};

// --------- //
// Structure //
// --------- //

pub struct Logger {
	pub(super) colorized: bool,
	pub(super) format_fn: FormatFn,
	pub(super) filters_fn: Vec<Box<FilterFn>>,
	pub(super) level: Option<LevelFilter>,
}

// -------------- //
// Implémentation //
// -------------- //

impl Logger {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub fn apply(self) -> Result<(), SetLoggerError> {
		let level = self.level.unwrap_or(LevelFilter::Off);
		log::set_max_level(level);
		log::set_boxed_logger(Box::new(self))
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Log for Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() != LevelFilter::Off
			&& self.filters_fn.iter().all(|once_fn| once_fn(metadata))
	}

	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let message = record.args();
		if message.to_string().trim().is_empty() {
			return;
		}

		let level = record.level().to_string();
		let level = if self.colorized {
			level
		} else {
			match level.as_ref() {
				| "INFO" => " INFO",
				| "WARN" => " WARN",
				| o => o,
			}
			.to_string()
		};

		let mut echo = Echo {
			delimiter: "|".to_owned(),
			record_level: record.level(),
			level: if self.colorized {
				format!("%c{level}%c")
			} else {
				level
			},
			_marker: Default::default(),
		};

		let render = (self.format_fn)(&mut echo, message, record);
		let render = JsValue::from_str(&render);

		let base = "\
			 padding: 5px 10px; \
			 margin-bottom: 2px; \
			 border-radius: 3px; \
		 ";
		let unset_bg = "\
			 background-color: unset; \
		 ";
		let file_bold = "\
			 font-weight: bold; \
			 color: #29323d; \
		 ";
		let unset_bold = "\
			 font-weight: normal; \
			 color: inherit; \
		 ";

		match record.level() {
			| Level::Error => {
				if self.colorized {
					let style = "
						 background: #95324d; \
						 border: 1px solid #A14A62; \
						 color: #EAF2F4; \
					 ";
					console::error_5(
						&render,
						&JsValue::from_str(&format!("{base}{style}")),
						&JsValue::from_str(unset_bg),
						&JsValue::from_str(file_bold),
						&JsValue::from_str(unset_bold),
					);
				} else {
					console::error_1(&render);
				}
			}
			| Level::Warn => {
				if self.colorized {
					let style = "
						 background: #F39C12; \
						 border: 1px solid #D46A43; \
						 color: #FFF; \
					 ";

					console::warn_5(
						&render,
						&JsValue::from_str(&format!("{base}{style}")),
						&JsValue::from_str(unset_bg),
						&JsValue::from_str(file_bold),
						&JsValue::from_str(unset_bold),
					);
				} else {
					console::warn_1(&render);
				}
			}
			| Level::Info => {
				if self.colorized {
					let style = "
						 background: #3FB0CC; \
						 border: 1px solid #2590ab; \
						 color: #EAF2F4; \
					 ";
					console::info_5(
						&render,
						&JsValue::from_str(&format!("{base}{style}")),
						&JsValue::from_str(unset_bg),
						&JsValue::from_str(file_bold),
						&JsValue::from_str(unset_bold),
					);
				} else {
					console::info_1(&render);
				}
			}
			| Level::Debug => {
				if self.colorized {
					let style = "
						 background: #22b780; \
						 border: 1px solid #17885e; \
						 color: #EAF2F4; \
					 ";
					console::debug_5(
						&render,
						&JsValue::from_str(&format!("{base}{style}")),
						&JsValue::from_str(unset_bg),
						&JsValue::from_str(file_bold),
						&JsValue::from_str(unset_bold),
					);
				} else {
					console::debug_1(&render);
				}
			}
			| Level::Trace => {
				if self.colorized {
					let style = "
						 background: #29323d; \
						 border: 1px solid #A14A62; \
						 color: #EAF2F4; \
					 ";
					console::trace_5(
						&render,
						&JsValue::from_str(&format!("{}{}", base, style)),
						&JsValue::from_str(unset_bg),
						&JsValue::from_str(file_bold),
						&JsValue::from_str(unset_bold),
					);
				} else {
					console::trace_1(&render);
				}
			}
		};
	}

	fn flush(&self) {}
}
