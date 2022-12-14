/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod builder;
mod state;

use core::fmt;
use std::{
	collections::HashMap,
	ops,
	str::{Chars, FromStr},
};

use lang::stream::{InputStream, InputStreamError};

use crate::tags::builder::Builder;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize)]
pub struct Tags(pub HashMap<String, String>);

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Error {
	InputStream,
	InvalidCharacter { found: char, help: &'static str },
	Parse,
	IsNotStartingWithCommercialChar,
	KeyIsEmpty,
	ValueIsEmpty,
}

// -------------- //
// Implémentation //
// -------------- //

impl Tags {
	pub(super) fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, Error> {
		let mut builder = Builder::initial(stream);
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

impl<const N: usize, K, V> From<[(K, V); N]> for Tags
where
	K: Into<String> + PartialEq + Eq + Clone,
	V: Into<String> + PartialEq + Eq + Clone,
{
	fn from(sized: [(K, V); N]) -> Self {
		let it: HashMap<String, String> =
			sized.map(|(a, b)| (a.into(), b.into())).into();
		Self(HashMap::from_iter(it))
	}
}

impl ops::Deref for Tags {
	type Target = HashMap<String, String>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for Tags {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<InputStreamError> for Error {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream => "erreur dans le flux".to_owned(),
				| Self::Parse => "erreur d'analyse".to_owned(),
				| Self::InvalidCharacter { found, .. } =>
					format!("caractère {found} invalide"),
				| Self::IsNotStartingWithCommercialChar =>
					"ne commence pas par un caractère commercial (@)".to_owned(),
				| Self::KeyIsEmpty => "le nom de la clé est vide".to_owned(),
				| Self::ValueIsEmpty => "la valeur est vide".to_owned(),
			}
		)
	}
}

impl FromStr for Error {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.ends_with("clé vide") {
			return Ok(Self::KeyIsEmpty);
		} else if s.contains("caractère invalide -> ") {
			let parts = unsafe {
				s.split_once(" -> ")
					.map(|(_, x)| {
						x.replace("\\s", " ")
							.replace("\\r", "\r")
							.replace("\\n", "\n")
					})
					.unwrap_unchecked()
			};
			let found = parts.as_bytes();
			return Ok(Self::InvalidCharacter {
				found: found[0] as char,
				help: "Un point de code valide est attendu",
			});
		}

		Err("non géré")
	}
}

impl fmt::Display for Tags {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut output = String::new();

		if !self.is_empty() {
			output.push('@');
		}

		self.iter().for_each(|(k, v)| {
			output.push_str(k);
			output.push('=');
			output.push_str(v);
			output.push(';');
		});

		write!(f, "{output}")
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(input: &str) -> Result<Tags, Error> {
		let mut stream = InputStream::new(input.chars());
		Tags::parse(&mut stream)
	}

	#[test]
	fn test_tags_ok() {
		let input = "@admin";
		let output = parse(input).unwrap();
		assert_eq!(output, [("admin", "true")].into());

		let input = "@id=1;first-name=Mike";
		let output = parse(input).unwrap();
		assert_eq!(output, [("id", "1"), ("first-name", "Mike"),].into());

		let input = "@example.org/foo=bar";
		let output = parse(input).unwrap();
		assert_eq!(output, [("example.org/foo", "bar")].into());

		let input = "@+icon=https://example.com/favicon.png";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			[("+icon", "https://example.com/favicon.png")].into()
		);

		let input =
			"@time=2021-01-27T18:09:19.337Z;msgid=SiCzdPygaGoToMz8Jg9gLS";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			[
				("time", "2021-01-27T18:09:19.337Z"),
				("msgid", "SiCzdPygaGoToMz8Jg9gLS"),
			]
			.into()
		);

		let input = "@a=b;c=d;";
		let output = parse(input);
		assert!(output.is_ok());
	}

	#[test]
	fn test_tags_error() {
		let input = "@=";
		let output = parse(input);
		assert!(output.is_err());

		let input = "@a=";
		let output = parse(input);
		assert!(output.is_err());
	}
}
