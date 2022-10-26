/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{ops, str::Chars};

use lang::stream::InputStream;

use crate::{
	command::builder::ParseCommandParametersBuilder, MessageCommandError,
};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize)]
pub struct MessageCommandParameters(pub Vec<String>);

// -------------- //
// Implémentation //
// -------------- //

impl MessageCommandParameters {
	pub(super) fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, MessageCommandError> {
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
impl<const N: usize, T> From<[T; N]> for MessageCommandParameters
where
	T: Into<String>,
	T: Clone,
{
	fn from(sized: [T; N]) -> Self {
		Self(sized.map(Into::into).to_vec())
	}
}

impl ops::Deref for MessageCommandParameters {
	type Target = [String];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for MessageCommandParameters {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
