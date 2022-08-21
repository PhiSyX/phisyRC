/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum IrcCommandNumeric {
	ERR_UNKNOWNCOMMAND { command: String },
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcCommandNumeric {
	pub fn code(&self) -> u16 {
		match self {
			Self::ERR_UNKNOWNCOMMAND { .. } => 421,
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for IrcCommandNumeric {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::ERR_UNKNOWNCOMMAND { command } =>
					format!("{command} :Unknown command"),
			}
		)
	}
}
