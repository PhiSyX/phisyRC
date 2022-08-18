/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

pub type IrcResult<T> = Result<T, IrcError>;

// ----------- //
// Erreurs IRC //
// ----------- //

#[derive(Debug)]
pub enum IrcError {
	IO(std::io::Error),

	ParseConfigError(toml::de::Error),
}

// -------------- //
// Implémentation // -> Gestion des erreurs (`?`)
// -------------- //

impl From<std::io::Error> for IrcError {
	fn from(err: std::io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<toml::de::Error> for IrcError {
	fn from(err: toml::de::Error) -> Self {
		Self::ParseConfigError(err)
	}
}

impl fmt::Display for IrcError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::IO(io_err) => {
					format!("{io_err}")
				}
				| Self::ParseConfigError(toml_err) => {
					format!("[ParseConfigError]: {}", toml_err)
				}
			}
		)
	}
}
