/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use log::{LevelFilter, SetLoggerError};

use crate::{FilterFn, FormatFn};

// --------- //
// Structure //
// --------- //

#[derive(Default)]
pub struct Builder {
	colorized: bool,
	timestamp: bool,
	level: Option<LevelFilter>,
	format_fn: Option<FormatFn>,
	filters_fn: Vec<Box<FilterFn>>,
}

impl Builder {
	/// Ajoute un filtre au système de log.
	pub fn filter<F>(mut self, predicate: F) -> Self
	where
		F: 'static,
		F: Send,
		F: Sync,
		F: Fn(&log::Metadata) -> bool,
	{
		self.filters_fn.push(Box::new(predicate));
		self
	}

	/// Autorise les logs à être colorés.
	pub fn with_color(mut self) -> Self {
		self.colorized = true;
		self
	}

	/// Le format du log.
	pub fn with_format(mut self, format: FormatFn) -> Self {
		self.format_fn.replace(format);
		self
	}

	/// Le niveau de log.
	pub fn with_level(mut self, level: LevelFilter) -> Self {
		self.level.replace(level);
		self
	}

	/// Autorise les logs à avoir un timestamp.
	pub fn with_timestamp(mut self) -> Self {
		self.timestamp = true;
		self
	}

	#[cfg(feature = "stdout")]
	/// Construction du logger (normal)
	pub fn build_stdout(self) -> Result<(), SetLoggerError> {
		use terminal::{
			format::color::Interface,
			layout::{Alignment, Cell},
		};

		use crate::stdout;

		stdout::Logger {
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
							record.target().gray(),
							"->".red(),
							message
						)),
					]);
				} else {
					echo.table.add_line([
						Cell::new(&echo.level).with_alignment(Alignment::Right),
						Cell::new(&echo.delimiter),
						Cell::new(record.target().gray()),
						Cell::new("->".red()),
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

	#[cfg(feature = "tui")]
	/// Construction du logger (tui).
	pub async fn build_tui<Ctx>(
		self,
		ctx: tokio::sync::mpsc::UnboundedSender<Ctx>,
	) -> std::io::Result<()>
	where
		Ctx: terminal::EventLoop,
	{
		use crate::tui;

		let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
		tui::Logger {
			colorized: self.colorized,
			timestamp: self.timestamp,
			level: self.level,
			filters_fn: self.filters_fn,
			writer: tx,
		}
		.apply()
		.expect("Le logger ne DOIT pas s'initialiser plusieurs fois.");

		tokio::spawn(tui::Tui::launch(ctx, rx));

		Ok(())
	}

	#[cfg(feature = "wasm")]
	pub fn build_wasm(self) -> Result<(), SetLoggerError> {
		use std::fmt::Arguments;

		use log::Record;

		use crate::{echo::Echo, wasm};

		wasm::Logger {
			colorized: self.colorized,
			format_fn: self.format_fn.unwrap_or_else(|| {
				|echo: &mut Echo, message: &Arguments, record: &Record| {
					format!(
						"{} {} %c{} (line {})%c:\n\n\t{}\n",
						echo.level,
						echo.delimiter,
						record.file().unwrap_or_else(|| record.target()),
						record.line().unwrap_or(0),
						message
					)
				}
			}),
			filters_fn: self.filters_fn,
			level: self.level,
		}
		.apply()
	}
}
