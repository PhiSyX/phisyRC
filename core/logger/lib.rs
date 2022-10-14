/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod builder;
mod echo;
mod export;

use std::fmt::Arguments;

use chrono::Local;
use terminal::{format::Interface, layout::GridLayout};

pub use self::export::*;
use self::{builder::Builder, echo::Echo};

// ---- //
// Type //
// ---- //

type FormatFn = fn(&mut Echo, &Arguments, &Record) -> String;
type FilterFn = fn(&Metadata) -> bool;

// --------- //
// Structure //
// --------- //

pub struct Logger {
	colorized: bool,
	timestamp: bool,
	level: Option<LevelFilter>,

	format_fn: FormatFn,
	filters_fn: Vec<FilterFn>,
}

struct NopeLogger;

const NO: NopeLogger = NopeLogger;

// -------------- //
// Implémentation //
// -------------- //

impl Logger {
	pub fn builder() -> Builder {
		Builder::default()
	}

	fn apply(self) -> Result<(), SetLoggerError> {
		let level = self.level.unwrap_or(LevelFilter::Off);
		set_max_level(level);
		if LevelFilter::Off == max_level() {
			set_logger(&NO)
		} else {
			set_boxed_logger(Box::new(self))
		}
	}
}

// -------------- //
// Implémentation // - Interface
// -------------- //

impl Log for Logger {
	/// On ne veut pas afficher les logs si le niveau est à
	/// [LevelFilter::Off].
	/// Des conditions utilisateurs peuvent être utilisées pour
	/// filtrer les logs.
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() != LevelFilter::Off
			&& self.filters_fn.iter().all(|boxed_fn| boxed_fn(metadata))
	}

	/// Affiche le log.
	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let message = record.args();
		if message.to_string().trim().is_empty() {
			println!();
			return;
		}

		let level = if self.colorized {
			match record.level() {
				| Level::Error => "ERROR".red(),
				| Level::Warn => " WARN".yellow(),
				| Level::Info => " INFO".blue(),
				| Level::Debug => "DEBUG".magenta(),
				| Level::Trace => "TRACE".white(),
			}
			.to_string()
		} else {
			record.level().to_string()
		};

		let mut table = GridLayout::default()
			// .define_max_width(80)
			.without_boarder();

		let mut echo = Echo {
			time: if self.timestamp {
				Some(Local::now())
			} else {
				None
			},
			delimiter: if self.colorized {
				"|".red().to_string()
			} else {
				"|".to_string()
			},
			level,
			record_level: record.level(),
			table: &mut table,
		};

		let text = (self.format_fn)(&mut echo, message, record);

		echo.log(text);
	}

	fn flush(&self) {}
}

impl Log for NopeLogger {
	fn enabled(&self, _: &Metadata) -> bool {
		false
	}

	fn log(&self, _: &Record) {}

	fn flush(&self) {}
}
