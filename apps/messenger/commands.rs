/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use irc_replies::{text, IncomingCommand};

use crate::AppContext;

text! { impl ServerCommand
	<- SESSIONS
}

// -------------- //
// Implémentation //
// -------------- //

impl ServerCommand {
	pub fn parse(input: &str) -> Result<Self, crate::Error> {
		let msg = irc_msg::Message::parse_from(format!("{input}\r\n"))?;
		Ok(msg.command.is_valid()?)
	}

	pub fn handle(&self, server: &crate::NetworkServer<crate::AppServer>) {
		match self {
			| Self::SESSIONS { .. } => {
				server.notify(AppContext::SessionsList);
			}
		}
	}
}
