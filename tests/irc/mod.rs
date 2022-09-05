/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod message;

use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{World, WorldInit};
use irc::{IrcMessage, IrcMessageError};

// --------- //
// Structure //
// --------- //

#[derive(Debug, WorldInit)]
pub struct IrcWorld {
	current_message: Result<IrcMessage, IrcMessageError>,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[async_trait(?Send)]
impl World for IrcWorld {
	type Error = Infallible;

	async fn new() -> Result<Self, Self::Error> {
		Ok(Self {
			// Just for the test
			current_message: Err(IrcMessageError::InputStream),
		})
	}
}
