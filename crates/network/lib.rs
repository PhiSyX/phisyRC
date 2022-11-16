/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod client;
mod export;
pub mod server;
pub mod session;
mod socket;

use core::fmt;
use std::io;

pub use self::export::*;

// ---- //
// Type //
// ---- //

pub type AnyError = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T, E = Error> = std::result::Result<T, E>;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum Error {
	/// Erreur I/O.
	IO(io::Error),
	/// Erreur liée au WebSocket.
	WebSocket(tokio_tungstenite::tungstenite::Error),
}

// -------------- //
// Implémentation // -> From<T>
// -------------- //

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
	fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
		Self::WebSocket(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let err_s = match self {
			| Self::IO(err) => err.to_string(),
			| Self::WebSocket(err) => err.to_string(),
		};
		write!(f, "{err_s}")
	}
}
