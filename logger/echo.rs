/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use chrono::{DateTime, Local};
use cli::layout::GridLayout;
use log::{Level, LevelFilter};

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

// -------------- //
// Implémentation //
// -------------- //

impl Echo<'_> {
	pub fn log(self, text: String) {
		if self.record_level == LevelFilter::Error {
			eprint!("{text}");
		} else {
			print!("{text}");
		}
	}
}
