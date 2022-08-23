/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::borrow::Cow;

// TODO(phisyx): utiliser une macro pour aller plus vite.

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum IrcCommandNumeric {
	ERR_UNKNOWNCOMMAND {
		command: String,
	},
	ERR_NONICKNAMEGIVEN,
	ERR_NICKNAMEINUSE {
		nick: String,
	},
	ERR_NOTREGISTERED,
	ERR_NEEDMOREPARAMS {
		command: String,
	},
	ERR_ALREADYREGISTRED,

	RPL_WELCOME {
		nick: String,
		user: String,
		host: String,
	},
	RPL_YOURHOST {
		servername: String,
		ver: String,
	},
	RPL_CREATED {
		date: String,
	},
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcCommandNumeric {
	pub fn code(&self) -> &'static str {
		match self {
			| Self::ERR_UNKNOWNCOMMAND { .. } => "421",
			| Self::ERR_NONICKNAMEGIVEN => "431",
			| Self::ERR_NICKNAMEINUSE { .. } => "433",
			| Self::ERR_NOTREGISTERED => "451",
			| Self::ERR_NEEDMOREPARAMS { .. } => "461",
			| Self::ERR_ALREADYREGISTRED => "462",

			| Self::RPL_WELCOME { .. } => "001",
			| Self::RPL_YOURHOST { .. } => "002",
			| Self::RPL_CREATED { .. } => "003",
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for IrcCommandNumeric {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let msg: Cow<str> = match self {
			| Self::ERR_UNKNOWNCOMMAND { command } => {
				Cow::from(format!("{command} :Unknown command"))
			}

			| Self::ERR_NONICKNAMEGIVEN => Cow::from(":No nickname given"),

			| Self::ERR_NICKNAMEINUSE { nick } => {
				Cow::from(format!("{nick}: Nickname is already in use"))
			}

			| Self::ERR_NOTREGISTERED => Cow::from(":You have not registered"),

			| Self::ERR_NEEDMOREPARAMS { command } => {
				Cow::from(format!("{command} :Not enough parameters"))
			}

			| Self::ERR_ALREADYREGISTRED => {
				Cow::from(":Unauthorized command (already registered)")
			}

			| Self::RPL_WELCOME { nick, user, host } => Cow::from(format!(
				":Welcome to the Internet Relay Network {nick}!{user}@{host}"
			)),

			| Self::RPL_YOURHOST { servername, ver } => Cow::from(format!(
				"Your host is {servername}, running version {ver}"
			)),

			| Self::RPL_CREATED { date } => {
				Cow::from(format!(":This server was created {date}"))
			}
		};

		write!(f, "{}", msg)
	}
}
