/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use cli::{
	layout::{Alignment, Cell},
	style::Stylize,
};
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
				let local_date_format =
					echo.time.map(|t| t.format("%Y-%m-%d@%H:%M:%S"));

				if let Some(time) = local_date_format {
					echo.table.add_line([
						Cell::new(&echo.level).with_alignment(Alignment::Right),
						Cell::new(&echo.delimiter),
						Cell::new(time),
						Cell::new(&echo.delimiter),
						Cell::new(format!(
							"{} {} {}",
							record.target().dark_grey(),
							"->".dark_red(),
							message
						)),
					]);
				} else {
					echo.table.add_line([
						Cell::new(&echo.level).with_alignment(Alignment::Right),
						Cell::new(&echo.delimiter),
						Cell::new(record.target().dark_grey()),
						Cell::new("->".dark_red()),
						Cell::new(message),
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
