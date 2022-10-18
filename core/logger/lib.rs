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

use std::str::FromStr;

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

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum LoggerType {
	#[default]
	Stdout,
	Tui,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl FromStr for LoggerType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			| "tui" => Self::Tui,
			| _ => Self::Stdout,
		})
	}
}

impl<'a> From<&'a str> for LoggerType {
	fn from(s: &'a str) -> Self {
		s.parse().unwrap_or_default()
	}
}

impl Log for NopeLogger {
	fn enabled(&self, _: &Metadata) -> bool {
		false
	}

	fn log(&self, _: &Record) {}

	fn flush(&self) {}
}
