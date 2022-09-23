/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// ----------- //
// Énumération //
// ----------- //

use crate::server::OutgoingReason;

#[derive(Debug)]
#[derive(Clone)]
pub enum SocketMessageRaw {
	Text(String),
	Binary(Vec<u8>),
	Ping(Vec<u8>),
	Pong(Vec<u8>),
	Close(Option<OutgoingReason>),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum SocketMessage {
	Text(String),
	Binary(Vec<u8>),
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<SocketMessage> for SocketMessageRaw {
	fn from(message: SocketMessage) -> Self {
		match message {
			| SocketMessage::Text(text) => Self::Text(text),
			| SocketMessage::Binary(bytes) => Self::Binary(bytes),
		}
	}
}

impl From<SocketMessageRaw> for String {
	fn from(s: SocketMessageRaw) -> Self {
		if let SocketMessageRaw::Text(s) = s {
			format!("{s}\r\n")
		} else {
			String::default()
		}
	}
}

impl From<String> for SocketMessageRaw {
	fn from(s: String) -> Self {
		Self::Text(format!("{s}\r\n"))
	}
}
