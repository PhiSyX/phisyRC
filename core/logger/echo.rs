/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use log::{Level, LevelFilter, Record};

// --------- //
// Structure //
// --------- //

pub struct Echo<'a> {
	#[cfg(any(feature = "stdout", feature = "tui"))]
	pub(super) time: Option<chrono::DateTime<chrono::Local>>,
	pub(super) delimiter: String,
	pub(super) level: String,
	pub(super) record_level: Level,
	#[cfg(any(feature = "stdout", feature = "tui"))]
	pub(super) table: &'a mut terminal::layout::GridLayout<'a>,
	#[cfg(feature = "wasm")]
	pub(super) _marker: std::marker::PhantomData<&'a ()>,
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

#[cfg(any(feature = "stdout", feature = "tui"))]
pub type LoggerWriter = tokio::sync::mpsc::UnboundedSender<Entry>;
#[cfg(any(feature = "stdout", feature = "tui"))]
pub type LoggerReader = tokio::sync::mpsc::UnboundedReceiver<Entry>;

// -------------- //
// Implémentation //
// -------------- //

impl Echo<'_> {
	#[cfg(feature = "stdout")]
	/// `Stdout`: Affichage du log.
	pub(super) fn log(self, text: String) {
		if self.record_level == LevelFilter::Error {
			eprint!("{text}");
		} else {
			print!("{text}");
		}
	}
}

impl Entry {
	#[cfg(feature = "tui")]
	/// `TUI`: le style d'un log.
	pub(super) fn style(&self) -> terminal::tui::style::Style {
		use terminal::tui::style::{Color, Style};

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
