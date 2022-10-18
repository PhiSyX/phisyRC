/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use chrono::{DateTime, Local};
use log::{Level, LevelFilter, Record};
use terminal::{
	layout::GridLayout,
	tui::style::{Color, Style},
};
use tokio::sync::mpsc;

// --------- //
// Structure //
// --------- //

pub struct Echo<'a> {
	pub(super) time: Option<DateTime<Local>>,
	pub(super) delimiter: String,
	pub(super) level: String,
	pub(super) record_level: Level,
	pub(super) table: &'a mut GridLayout<'a>,
}

#[derive(Debug)]
pub struct Entry {
	pub(super) level: LevelFilter,
	pub(super) target: String,
	pub(super) args: String,
}

// ---- //
// Type //
// ---- //

pub type LoggerWriter = mpsc::UnboundedSender<Entry>;
pub type LoggerReader = mpsc::UnboundedReceiver<Entry>;

// -------------- //
// Implémentation //
// -------------- //

impl Echo<'_> {
	pub(super) fn log(self, text: String) {
		if self.record_level == LevelFilter::Error {
			eprint!("{text}");
		} else {
			print!("{text}");
		}
	}
}

impl Entry {
	pub(super) fn style(&self) -> Style {
		match self.level {
			| LevelFilter::Off => Style::default(),
			| LevelFilter::Error => Style::default().fg(Color::Red),
			| LevelFilter::Warn => Style::default().fg(Color::Yellow),
			| LevelFilter::Info => Style::default().fg(Color::Blue),
			| LevelFilter::Debug => Style::default().fg(Color::Magenta),
			| LevelFilter::Trace => Style::default().fg(Color::White),
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<&Record<'_>> for Entry {
	fn from(record: &Record) -> Self {
		Self {
			level: record.level().to_level_filter(),
			target: record.target().to_string(),
			args: record.args().to_string(),
		}
	}
}
