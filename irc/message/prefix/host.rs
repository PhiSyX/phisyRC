/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::net::IpAddr;

use lang::{codepoints::CodePoint, lexer::ParseState, stream::prelude::*};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub(crate) enum IrcMessagePrefixHostError {
	InputStream,
	IsEmpty,
	InvalidFirstCharacter,
	InvalidCharacter,
	InvalidLastCharacter,
}

// ------ //
// Parser //
// ------ //

/// Analyse d'un nom d'hôte ou d'adresse IP.
//
// BNF: <host>     ::=  hostname / hostaddr
//      <hostname>  ::=  shortname *( "." shortname )
//      <shortname> ::=  ( letter / digit ) *( letter / digit / "-" )
//                  ::=  *( letter / digit ) ; as specified in RFC 1123 [HNAME]
//      <hostaddr>  ::=  ip4addr / ip6addr
//      <ip4addr>   ::=  1*3digit "." 1*3digit "." 1*3digit "." 1*3digit
//      <ip6addr>   ::=  1*hexdigit 7( ":" 1*hexdigit )
//                  ::=/ "0:0:0:0:0:" ( "0" / "FFFF" ) ":" ip4addr
//
// NOTE(phisyx): certains serveurs IRC acceptent le caractère '/' dans le nom
// d'hôte.
pub(super) fn parse(input: &str) -> Result<String, IrcMessagePrefixHostError> {
	let bytes: ByteStream = ByteStream::from(input);
	let mut stream = InputStream::new(bytes.chars());

	enum State {
		Initial,

		Hostname,
		HostnameAfterFirstCharacter,
	}

	impl ParseState for State {
		fn switch(&mut self, new_state: Self) {
			*self = new_state;
		}
	}

	let mut state = State::Initial;
	let mut host = String::new();

	loop {
		match state {
			| State::Initial => {
				if input.is_empty() {
					return Err(IrcMessagePrefixHostError::IsEmpty);
				}

				// NOTE(phisyx): ceci gère le cas `<hostaddr>`.
				if input.parse::<IpAddr>().is_ok() {
					return Ok(input.to_owned());
				}

				state.switch(State::Hostname);
			}

			| State::Hostname => match stream.consume_next()? {
				// Point de code alphabétique.
				//
				// Ajouter le point de code alphabétique au nom d'hôte.
				// Passer à l'état []
				| CodePoint::Unit(ch) if ch.is_alphanumeric() => {
					host.push(ch);
					state.switch(State::HostnameAfterFirstCharacter);
				}

				// Tous les autres points de code.
				//
				// Il s'agit d'une erreur d'analyse.
				| _ => {
					return Err(
						IrcMessagePrefixHostError::InvalidFirstCharacter,
					)
				}
			},

			| State::HostnameAfterFirstCharacter => {
				match stream.consume_next()? {
					// Point de code alphanumérique.
					//
					// Ajouter le point de code alphanumérique au nom d'hôte.
					| CodePoint::Unit(ch) if ch.is_alphanumeric() => {
						host.push(ch);
					}

					// U+002D HYPHEN-MINUS (-)
					// U+002E FULL STOP (.)
					// U+002F SOLIDUS (/)
					| codepoint @ (CodePoint::HYPHEN_MINUS
					| CodePoint::FULL_STOP
					| CodePoint::SOLIDUS) => {
						if stream.peek_next()?.is_eof() {
							return Err(
								IrcMessagePrefixHostError::InvalidLastCharacter,
							);
						}

						host.push(codepoint.unit());
					}

					| CodePoint::EOF if cfg!(test) => break,

					// Tous les autres points de code.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => {
						return Err(IrcMessagePrefixHostError::InvalidCharacter)
					}
				}
			}
		}
	}

	Ok(host)
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<InputStreamError> for IrcMessagePrefixHostError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl fmt::Display for IrcMessagePrefixHostError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream => "erreur d'analyse",
				| Self::IsEmpty => "le nom d'hôte / l'IP est vide",
				| Self::InvalidFirstCharacter =>
					"le nom d'hôte / l'IP commence par un caractère invalide",
				| Self::InvalidCharacter =>
					"le nom d'hôte / l'IP contient un caractère invalide",
				| Self::InvalidLastCharacter =>
					"le nom d'hôte / l'IP se termine par un caractère invalide",
			}
		)
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_localhost() {
		assert_eq!(parse("localhost"), Ok("localhost".to_owned()));
	}

	#[test]
	fn test_ipv4() {
		let input = "127.0.0.1";
		let output = parse(input).unwrap();
		assert_eq!(output, input);
	}

	#[test]
	fn test_ipv6() {
		let input = "::1";
		let output = parse(input).unwrap();
		assert_eq!(output, input);
	}

	#[test]
	fn test_hostname() {
		let input = "phisy.rc";
		let output = parse(input).unwrap();
		assert_eq!(output, input);

		let input = "gateway/web/dispatch/fake-ip.110.111.110.101";
		let output = parse(input).unwrap();
		assert_eq!(output, input);

		let input = "unafilliated/phisyx";
		let output = parse(input).unwrap();
		assert_eq!(output, input);

		let input = "Network-127-0-0-1.fai.country.net";
		let output = parse(input).unwrap();
		assert_eq!(output, input);
	}

	#[test]
	fn test_invalid_first_character() {
		let input = "@Network-127-0-0-1.fai.country.net";
		let output = parse(input);
		assert!(output.is_err());
	}

	#[test]
	fn test_invalid_last_character() {
		let input = "Network-127-0-0-1.fai.country.net-";
		let output = parse(input);
		assert!(output.is_err());

		let input = "Network-127-0-0-1.fai.country.net.";
		let output = parse(input);
		assert!(output.is_err());

		let input = "Network-127-0-0-1.fai.country.net@";
		let output = parse(input);
		assert!(output.is_err());
	}
}
