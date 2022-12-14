/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::str::FromStr;

use cucumber::{then, when};
use irc_msg::{command, prefix, tags, Error, Message};

use super::IrcWorld;
use crate::Bool;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum IrcMessageState {
	InvalidTags(tags::Error),
	InvalidPrefix(prefix::Error),
	InvalidCommand(command::Error),

	EmptyMessage,

	Valid,
}

impl FromStr for IrcMessageState {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.contains("(tags)") {
			return tags::Error::from_str(s).map(Self::InvalidTags);
		} else if s.contains("(préfixe)") {
			return prefix::Error::from_str(s).map(Self::InvalidPrefix);
		} else if s.contains("(commande)") {
			return command::Error::from_str(s).map(Self::InvalidCommand);
		}

		Ok(Self::Valid)
	}
}

// ---- //
// Test //
// ---- //

#[when(
	regex = r#"^on analyse (?:la ligne|un message)(?:\sIRC valide)? : "([^"]*)"$"#
)]
fn analyse_irc_line(w: &mut IrcWorld, line: String) {
	let line = line.replace("\\r", "\r").replace("\\n", "\n");
	let irc_msg = Message::parse_from(line);
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
			| Error::IsEmpty => IrcMessageState::EmptyMessage,
			| Error::InvalidTags(reason) => {
				IrcMessageState::InvalidTags(*reason)
			}
			| Error::InvalidPrefix(reason) => {
				IrcMessageState::InvalidPrefix(*reason)
			}
			| Error::InvalidCommand(reason) => {
				IrcMessageState::InvalidCommand(*reason)
			}
			| Error::InputStream => IrcMessageState::EmptyMessage,
		},
	};

	assert_eq!(expected_state, state);
}

#[then(regex = r#"la présence de métadonnées est "([^"]+)"$"#)]
fn presence_of_metadata_is(w: &mut IrcWorld, conditional: Bool) {
	let msg = w.current_message.as_ref().unwrap();
	assert!(!msg.tags.is_empty() == conditional);
}

#[allow(clippy::nonminimal_bool)]
#[then(regex = r#"la présence d'un préfixe est "([^"]+)"$"#)]
fn presence_of_prefix_is(w: &mut IrcWorld, conditional: Bool) {
	let maybe_msg = w.current_message.as_ref();

	if false == conditional {
		if let Ok(msg) = maybe_msg {
			assert!(msg.prefix.is_none())
		} else {
			assert!(maybe_msg.is_err());
		}
		return;
	}

	let msg = maybe_msg.unwrap();
	assert!(msg.prefix.is_some() == conditional);
}

#[then(regex = r#"les métadonnées du message sont `([^`]+)`$"#)]
fn metadata_is(w: &mut IrcWorld, expected_metadata: serde_json::Value) {
	let msg = w.current_message.as_ref().unwrap();
	let json_tags = msg.tags.json();
	assert!(
		json_tags == expected_metadata,
		"Données réelles des tags en JSON: {}",
		json_tags
	);
}

// NOTE(phisyx): en admettant que des tests plus haut ont déjà fait une
// vérification pour ce que ce test puissent fonctionner correctement.
#[then(regex = r#"le préfixe du message est "([^"]*)"$"#)]
fn prefix_is(w: &mut IrcWorld, expected_prefix: String) {
	let maybe_msg = w.current_message.as_ref();
	if let Ok(msg) = maybe_msg {
		if let Some(prefix) = &msg.prefix {
			let expected_prefix =
				prefix::Prefix::parse_from_str(format!("{expected_prefix} "))
					.unwrap();
			assert!(expected_prefix.eq(prefix));
		}
	}
}

#[then(regex = r#"la commande du message est "([^"]*)"$"#)]
fn command_is(w: &mut IrcWorld, expected_command: String) {
	let msg = w.current_message.as_ref().unwrap();

	if let command::Command::Numeric { code, .. } = &msg.command {
		assert!(expected_command.eq(code));
	}

	if let command::Command::Text { command, .. } = &msg.command {
		assert!(expected_command.eq(command));
	}
}

#[then(regex = r#"les paramètres de la commande sont: `([^`]*)`$"#)]
fn parameters_of_command_is(
	w: &mut IrcWorld,
	expected_parameters: serde_json::Value,
) {
	let msg = w.current_message.as_ref().unwrap();

	if let command::Command::Text { parameters, .. } = &msg.command {
		let parameters_json = parameters.json();
		assert!(
			parameters_json == expected_parameters,
			"Données réelles des paramètres en JSON: {}",
			parameters_json
		);
	}
}
