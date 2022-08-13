/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use cli::layout::{Alignment, Cell};
use log::{LevelFilter, SetLoggerError};

use crate::{FilterFn, FormatFn, Logger};

#[derive(Default)]
pub struct Builder {
	colorized: bool,
	timestamp: bool,
	level: Option<LevelFilter>,

	format_fn: Option<FormatFn>,
	filters_fn: Vec<FilterFn>,
}

impl Builder {
	pub fn with_color(mut self) -> Self {
		self.colorized = true;
		self
	}

	pub fn with_format(mut self, format: FormatFn) -> Self {
		self.format_fn.replace(format);
		self
	}

	pub fn with_level(mut self, level: LevelFilter) -> Self {
		self.level.replace(level);
		self
	}

	pub fn with_timestamp(mut self) -> Self {
		self.timestamp = true;
		self
	}

	pub fn build(self) -> Result<(), SetLoggerError> {
		Logger {
			colorized: self.colorized,
			timestamp: self.timestamp,
			format_fn: self.format_fn.unwrap_or(|echo, message, record| {
				let time = echo.time.map(|t| t.format("%Y-%m-%d@%H:%M:%S"));

				let col_level =
					Cell::new(&echo.level).with_alignment(Alignment::Right);

				let col_target = Cell::new(record.target());
				let col_message = Cell::new(message);

				let delimiter_1 = Cell::new(&echo.delimiter);
				let delimiter_2 = Cell::new(':');

				if let Some(time) = time {
					let time = Cell::new(time);
					echo.table.add_row([
						col_level,
						delimiter_1.clone(),
						time,
						delimiter_1,
						col_target,
						delimiter_2,
						col_message,
					]);
				} else {
					echo.table.add_row([
						col_level,
						delimiter_1,
						col_target,
						delimiter_2,
						col_message,
					]);
				}

				echo.table.render()
			}),
			level: self.level,
			filters_fn: self.filters_fn,
		}
		.apply()
	}
}
