/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use logger::{wasm::Logger, LevelFilter};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

#[wasm_bindgen]
pub fn wasm_initialize_logger(log_level: String) {
	let log_level = log_level.parse().unwrap_or(LevelFilter::Trace);

	if let Err(err) = Logger::builder()
		.with_level(log_level)
		.with_color()
		.build_wasm()
	{
		let err_s = format!(
			"Impossible d'initialiser le loggeur plusieurs fois : {err}"
		);
		error(&err_s);
	}
}

#[wasm_bindgen]
pub fn logger_debug(arr: js_sys::Array) {
	let s = arr
		.iter()
		.map(|v| v.as_string())
		.flatten()
		.collect::<Vec<_>>()
		.join(" ");
	logger::debug!("{s}")
}

#[wasm_bindgen]
pub fn logger_info(arr: js_sys::Array) {
	let s = arr
		.iter()
		.map(|v| v.as_string())
		.flatten()
		.collect::<Vec<_>>()
		.join(" ");
	logger::info!("{s}")
}

#[wasm_bindgen]
pub fn logger_warn(arr: js_sys::Array) {
	let s = arr
		.iter()
		.map(|v| v.as_string())
		.flatten()
		.collect::<Vec<_>>()
		.join(" ");
	logger::warn!("{s}")
}

#[wasm_bindgen]
pub fn logger_error(arr: js_sys::Array) {
	let s = arr
		.iter()
		.map(|v| v.as_string())
		.flatten()
		.collect::<Vec<_>>()
		.join(" ");
	logger::error!("{s}")
}

#[wasm_bindgen]
pub fn logger_trace(arr: js_sys::Array) {
	let s = arr
		.iter()
		.map(|v| v.as_string())
		.flatten()
		.collect::<Vec<_>>()
		.join(" ");
	logger::trace!("{s}")
}
