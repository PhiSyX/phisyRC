/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use tokio_tungstenite::tungstenite;

use super::SocketMessageRaw;
use crate::{server::OutgoingReason, SocketMessage};

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<SocketMessageRaw> for tungstenite::Message {
	fn from(message: SocketMessageRaw) -> Self {
		match message {
			| SocketMessageRaw::Text(text) => Self::Text(text),
			| SocketMessageRaw::Binary(bytes) => Self::Binary(bytes),
			| SocketMessageRaw::Ping(bytes) => Self::Ping(bytes),
			| SocketMessageRaw::Pong(bytes) => Self::Pong(bytes),
			| SocketMessageRaw::Close(reason) => {
				Self::Close(reason.map(Into::into))
			}
		}
	}
}

impl From<tungstenite::Message> for SocketMessageRaw {
	fn from(message: tungstenite::Message) -> Self {
		match message {
			| tungstenite::Message::Text(text) => Self::Text(text),
			| tungstenite::Message::Binary(bytes) => Self::Binary(bytes),
			| tungstenite::Message::Ping(bytes) => Self::Ping(bytes),
			| tungstenite::Message::Pong(bytes) => Self::Pong(bytes),
			| tungstenite::Message::Close(Some(reason)) => {
				Self::Close(Some(reason.into()))
			}
			| tungstenite::Message::Close(None) => Self::Close(None),
			| tungstenite::Message::Frame(_) => {
				unreachable!("tungstenite frame")
			}
		}
	}
}
impl From<SocketMessage> for tungstenite::Message {
	fn from(message: SocketMessage) -> Self {
		match message {
			| SocketMessage::Text(text) => tungstenite::Message::Text(text),
			| SocketMessage::Binary(bytes) => {
				tungstenite::Message::Binary(bytes)
			}
		}
	}
}

impl<'t> From<OutgoingReason> for tungstenite::protocol::CloseFrame<'t> {
	fn from(reason: OutgoingReason) -> Self {
		Self {
			code: reason.code.into(),
			reason: reason.text.into(),
		}
	}
}

impl<'t> From<tungstenite::protocol::CloseFrame<'t>> for OutgoingReason {
	fn from(reason: tungstenite::protocol::CloseFrame<'t>) -> Self {
		Self {
			code: reason.code.into(),
			text: reason.reason.into(),
		}
	}
}
