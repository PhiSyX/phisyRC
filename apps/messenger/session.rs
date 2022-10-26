/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lang::stream::ByteStream;

use crate::{
	server::Server as AppServer, AppContext, NetworkServer, NetworkSession,
};

// ---- //
// Type //
// ---- //

pub type SessionID = uuid::Uuid;

pub type Session = NetworkSession<SessionID>;

// --------- //
// Structure //
// --------- //

#[derive(Clone)]
pub struct Actor {
	id: SessionID,
	server: NetworkServer<AppServer>,
}

// -------------- //
// Implémentation //
// -------------- //

impl Actor {
	pub fn new(
		server_instance: NetworkServer<AppServer>,
		id: SessionID,
	) -> Self {
		Self {
			server: server_instance,
			id,
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[network::async_trait]
impl network::session::Interface for Actor {
	type ID = SessionID;

	async fn binary(&mut self, bytes: Vec<u8>) -> network::Result<()> {
		let bytes_stream = ByteStream::from(bytes);

		// Vérifie que le message est de type IRC
		if let Ok(message) = irc_msg::Message::parse_from(bytes_stream) {
			// Reply...
			self.server.notify(AppContext::IRC(message));
		}

		Ok(())
	}
}
