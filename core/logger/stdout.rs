/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Arguments;

use chrono::Local;
use log::{
	max_level, set_boxed_logger, set_logger, set_max_level, Level, LevelFilter,
	Log, Metadata, Record, SetLoggerError,
};
use terminal::{format::color::Interface, layout::GridLayout};

use crate::{builder::Builder, echo::Echo, FilterFn, NO};

// ---- //
// Type //
// ---- //

pub(super) type FormatFn = fn(&mut Echo, &Arguments, &Record) -> String;

// --------- //
// Structure //
// --------- //

pub struct Logger {
	pub colorized: bool,
	pub timestamp: bool,
	pub level: Option<LevelFilter>,
	pub format_fn: FormatFn,
	pub filters_fn: Vec<Box<FilterFn>>,
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl Logger {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub fn apply(self) -> Result<(), SetLoggerError> {
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
			&& self.filters_fn.iter().all(|once_fn| once_fn(metadata))
	}

	/// Affiche le log.
	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let message = record.args();
		if message.to_string().trim().is_empty() {
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
				"|".red()
			} else {
				"|".to_owned()
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
