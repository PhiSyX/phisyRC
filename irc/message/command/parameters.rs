/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{ops, str::Chars};

use lang::stream::prelude::InputStream;

use crate::{
	message::command::builder::ParseCommandParametersBuilder,
	IrcMessageCommandError,
};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize)]
pub struct IrcMessageCommandParameters(pub Vec<String>);

// -------------- //
// Implémentation //
// -------------- //

impl IrcMessageCommandParameters {
	pub(super) fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, IrcMessageCommandError> {
		let mut builder = ParseCommandParametersBuilder::initialize(stream);
		builder.analyze()?;
		builder.finish()
	}

	#[cfg(feature = "json")]
	pub fn json(&self) -> serde_json::Value {
		serde_json::json!(self.0)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[cfg(test)] // NOTE(phisyx): code utilisé que lors des tests.
impl<const N: usize, T> From<[T; N]> for IrcMessageCommandParameters
where
	T: Into<String>,
	T: Clone,
{
	fn from(sized: [T; N]) -> Self {
		Self(sized.map(Into::into).to_vec())
	}
}

impl ops::Deref for IrcMessageCommandParameters {
	type Target = [String];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for IrcMessageCommandParameters {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
