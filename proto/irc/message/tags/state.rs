/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use lang::lexer::ParseState;

// ----------- //
// Énumération //
// ----------- //

#[derive(Default)]
pub(super) enum ParseTagsState {
	#[default]
	Initial,
	LeftKey,
	RightValue,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ParseState for ParseTagsState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}
