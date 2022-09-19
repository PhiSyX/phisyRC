/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use shared::err;

use crate::arch::{IrcNetworkError, IrcServerError};

// ----------- //
// Erreurs IRC //
// ----------- //

err! {
	| IO(std::io::Error) => "{}"
	| ParseConfigError(toml::de::Error) => "erreur de configuration -- {}"
	| Server(IrcServerError) => "serveur -- {}"
}

impl From<IrcNetworkError> for Error {
	fn from(err: IrcNetworkError) -> Self {
		match err {
			| IrcNetworkError::IO(err) => Self::IO(err),
			| IrcNetworkError::Server(err) => Self::Server(err),
		}
	}
}
