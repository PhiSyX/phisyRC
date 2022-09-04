/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use lang::lexer::ParseState;

// ----------- //
// Énumération //
// ----------- //

#[derive(Default)]
#[derive(PartialEq, Eq)]
pub(super) enum ParseCommandState {
	#[default]
	Initial,

	Numeric {
		counter: u8,
	},

	Text,
}

#[derive(Default)]
#[derive(PartialEq, Eq)]
pub(super) enum ParseCommandParametersFirstStepState {
	#[default]
	Initial,

	HasParameters,
}

#[derive(Default)]
#[derive(PartialEq, Eq)]
pub(super) enum ParseCommandParametersSecondStepState {
	#[default]
	Initial,

	AfterColon,
}

// -------------- //
// Implémentation //
// -------------- //

impl ParseCommandState {
	pub(super) fn increment_counter(&mut self) {
		if let ParseCommandState::Numeric { counter: count } = self {
			*count += 1;
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ParseState for ParseCommandState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}

impl ParseState for ParseCommandParametersFirstStepState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}

impl ParseState for ParseCommandParametersSecondStepState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}
