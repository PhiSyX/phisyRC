/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::commands::IrcCommandNumeric;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct IrcServer {
	registered: bool,
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcServer {
	pub fn new() -> Self {
		Self { registered: false }
	}
}

impl IrcServer {
	pub fn is_registered(&self) -> bool {
		self.registered
	}

	pub fn prefix(&self) -> Option<String> {
		todo!("prefix server")
	}

	pub fn prefix_based_on_reply(&self, _reply: &IrcCommandNumeric) -> String {
		todo!("prefix based on reply server")
	}
}
