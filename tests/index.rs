/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod irc;

use std::str::FromStr;

use cucumber::WorldInit;

use self::irc::IrcWorld;

#[tokio::main]
async fn main() {
	IrcWorld::run("./irc").await;
}

#[derive(Debug)]
#[repr(C)]
pub enum Bool {
	True = 1,
	False = 0,
}

impl FromStr for Bool {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			| "true" | "vrai" | "vraie" | "ok" | "oui" => Self::True,
			| "false" | "faux" | "fausse" | "ko" | "non" => Self::False,
			| _ => return Err("Valeur booléenne invalide"),
		})
	}
}

impl PartialEq<Bool> for bool {
	fn eq(&self, other: &Bool) -> bool {
		match self {
			| true => matches!(other, Bool::True),
			| false => matches!(other, Bool::False),
		}
	}
}
