/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use lang::lexer::ParseState;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Default)]
pub(super) enum ParsePrefixState {
	#[default]
	Initial,

	User,
	Server,
}

// -------------- //
// Implémentation //
// -------------- //

impl ParseState for ParsePrefixState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}
