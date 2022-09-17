/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod message;

use irc::{IrcMessage, IrcMessageError};

// --------- //
// Structure //
// --------- //

#[derive(Debug, cucumber::World)]
pub struct IrcWorld {
	current_message: Result<IrcMessage, IrcMessageError>,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Default for IrcWorld {
	fn default() -> Self {
		Self {
			current_message: Err(IrcMessageError::InputStream),
		}
	}
}
