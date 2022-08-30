/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use super::IrcCommandNumeric;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum IrcReplies {
	#[allow(dead_code)]
	Ignore,
	Error(String),
	NotifyChangeNick(String),
	NotifyChangeQuit(Option<String>),
	Numeric(IrcCommandNumeric),
}

// -------------- //
// Implémentation //
// -------------- //

impl From<IrcCommandNumeric> for IrcReplies {
	fn from(err: IrcCommandNumeric) -> Self {
		Self::Numeric(err)
	}
}
