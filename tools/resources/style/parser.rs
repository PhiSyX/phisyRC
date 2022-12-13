/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::str;
use std::collections::HashSet;

use lang::{
	lexer::{
		delimiter::{Assignment, Delimiter, Operator},
		literal::{Literal, StringOutput},
	},
	stream::{StreamIterator, StreamIteratorError, StreamIteratorItem},
};

use super::{LexerToken, TokenKind, TokenStream};

// --------- //
// Structure //
// --------- //

pub struct Parser;

// -------------- //
// Implémentation //
// -------------- //

impl Parser {
	pub fn parse<T>(mut token_stream: TokenStream) -> HashSet<T>
	where
		T: str::FromStr,
		T: PartialEq + Eq + std::hash::Hash,
	{
		let mut tokens = HashSet::new();

		'stream: loop {
			match token_stream.consume_next() {
				| Ok(token) => match token.kind() {
					| TokenKind::EOF => break 'stream,
					| TokenKind::Ignore => continue 'stream,

					| TokenKind::Identifier(ident) if ident.eq("class") => {
						let [Ok(a), Ok(b)] = &token_stream.peek_n_next(2)[..] else {
						continue 'stream;
					};

						if let (
							LexerToken {
								kind:
									TokenKind::Delimiter(Delimiter::Operator(
										Operator::Assignment(Assignment::EQUAL),
									)),
								..
							},
							LexerToken {
								kind:
									TokenKind::Literal(Literal::String(
										StringOutput { data, .. },
									)),
								..
							},
						) = (a, b)
						{
							let list = data
								.split_ascii_whitespace()
								.filter_map(|s| s.parse::<T>().ok());
							tokens.extend(list);
						} else {
							continue 'stream;
						}
					}
					| _ => continue 'stream,
				},
				| Err(err) if err.is_eos() => break 'stream,
				| Err(_) => continue 'stream,
			};
		}

		tokens
	}
}
