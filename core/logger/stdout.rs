/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Arguments;

use chrono::Local;
use log::Record;
use terminal::{format::Interface, layout::GridLayout};

use crate::{echo::Echo, Logger};

impl Logger {
	pub(super) fn echo(
		&self,
		level: String,
		record: &Record,
		message: &Arguments,
	) {
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
}
