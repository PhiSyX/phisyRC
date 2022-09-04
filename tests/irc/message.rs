/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::str::FromStr;

use cucumber::{then, when};
use irc::{
	IrcMessage, IrcMessageCommandError, IrcMessageError, IrcMessagePrefixError,
	IrcMessageTagsError,
};

use super::IrcWorld;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum IrcMessageState {
	InvalidTags(IrcMessageTagsError),
	InvalidPrefix(IrcMessagePrefixError),
	InvalidCommand(IrcMessageCommandError),

	EmptyMessage,
	ParseError,

	Valid,
}

impl FromStr for IrcMessageState {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.contains("(tags)") {
			return IrcMessageTagsError::from_str(s).map(Self::InvalidTags);
		} else if s.contains("(préfixe)") {
			return IrcMessagePrefixError::from_str(s).map(Self::InvalidPrefix);
		} else if s.contains("(commande)") {
			return IrcMessageCommandError::from_str(s)
				.map(Self::InvalidCommand);
		}

		Ok(Self::Valid)
	}
}

// ---- //
// Test //
// ---- //

#[when(regex = r#"^on analyse la ligne : "([^"]*)"$"#)]
fn analyse_irc_line(w: &mut IrcWorld, line: String) {
	let line = line.replace("\\r", "\r").replace("\\n", "\n");
	let irc_msg = IrcMessage::parse_from_str(line);
	w.current_message = irc_msg;
}

#[then(regex = r"la ligne DOIT être considérée comme étant `([^`]+)`$")]
fn current_line_must_be_considered_as(
	w: &mut IrcWorld,
	expected_state: IrcMessageState,
) {
	let state = match &w.current_message {
		| Ok(_) => IrcMessageState::Valid,
		| Err(err) => match err {
			| IrcMessageError::IsEmpty => IrcMessageState::EmptyMessage,
			| IrcMessageError::InvalidTags(reason) => {
				IrcMessageState::InvalidTags(*reason)
			}
			| IrcMessageError::InvalidPrefix(reason) => {
				IrcMessageState::InvalidPrefix(*reason)
			}
			| IrcMessageError::InvalidCommand(reason) => {
				IrcMessageState::InvalidCommand(*reason)
			}
			| IrcMessageError::InputStream => IrcMessageState::EmptyMessage,
		},
	};

	assert_eq!(expected_state, state)
}
