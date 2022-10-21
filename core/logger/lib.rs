/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod builder;
mod echo;
mod export;
pub mod stdout;
pub mod tui;

pub use self::export::*;

// ---- //
// Type //
// ---- //

pub(crate) type FilterFn = dyn Fn(&log::Metadata) -> bool + Send + Sync;

// --------- //
// Constante //
// --------- //

pub(crate) const NO: NopeLogger = NopeLogger;

// --------- //
// Structure //
// --------- //

pub(crate) struct NopeLogger;

impl Log for NopeLogger {
	fn enabled(&self, _: &Metadata) -> bool {
		false
	}

	fn log(&self, _: &Record) {}

	fn flush(&self) {}
}
