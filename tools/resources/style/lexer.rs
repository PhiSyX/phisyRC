/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;
use std::{marker::PhantomData, ops};

use lang::{
	codepoints::{CodePoint, CodePointInterface},
	lexer::{
		delimiter::{Delimiter, DelimiterError, DelimiterOutput},
		identifier::{IdentifierError, IdentifierOutput, LexicalIdentifier},
		keyword::Keyword,
		literal::{LexicalString, Literal, StringError, StringOutput},
		Input, LexicalError, TokenizerAlgorithms,
	},
	location::{Location, LocationInterface},
	stream::{InputStream, StreamIterator, StreamIteratorItem},
};

use super::TokenStream;

// --------- //
// Structure //
// --------- //

pub struct Lexer;

pub struct Tokenizer<CodePoints, Token, TokenError> {
	input: CodePoints,
	location: Location,
	current_token: Option<Token>,
	reconsume_now: bool,
	_markerr: PhantomData<TokenError>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexerToken {
	pub(crate) kind: TokenKind,
	location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum TokenKind {
	Identifier(IdentifierOutput),
	Keyword(Keyword),
	Literal(Literal),
	Delimiter(Delimiter),
	EOF,
	Ignore,
}

struct LexicalDelimiter<'a, Input> {
	input: &'a mut Input,
	location: Location,
}

// -------------- //
// Implémentation //
// -------------- //

impl Lexer {
	pub fn lex<CodePoints, Unit>(source: CodePoints) -> TokenStream
	where
		CodePoints: Iterator<Item = Unit>,
		Unit: CodePointInterface,
	{
		let codepoints_stream: InputStream<CodePoints, Unit> =
			InputStream::new(source);
		Self::tokenize(codepoints_stream).stream()
	}

	fn tokenize<C>(codepoints: C) -> Tokenizer<C, LexerToken, LexicalError> {
		Tokenizer::new(codepoints)
	}
}

impl LexerToken {
	fn new(kind: TokenKind, location: Location) -> Self {
		Self { kind, location }
	}
}

impl<C, T, E> Tokenizer<C, T, E> {
	fn new(codepoints: C) -> Self {
		Self {
			input: codepoints,
			location: Location::new(),
			current_token: Default::default(),
			reconsume_now: false,
			_markerr: Default::default(),
		}
	}
}

impl<C, U> Tokenizer<C, LexerToken, LexicalError>
where
	C: StreamIterator<Item = CodePoint<U>>,
	U: CodePointInterface,
{
	fn stream(self) -> TokenStream {
		TokenStream::from_stream(self)
	}
}

impl<'a, I> LexicalDelimiter<'a, I> {
	fn new(input: &'a mut I) -> Self {
		Self {
			input,
			location: Default::default(),
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<C, U> StreamIterator for Tokenizer<C, LexerToken, LexicalError>
where
	C: StreamIterator<Item = CodePoint<U>>,
	U: CodePointInterface,
{
	type Error = LexicalError;
	type Item = LexerToken;

	fn current(&self) -> Option<Self::Item> {
		self.current_token.clone()
	}

	fn consume_next(&mut self) -> Result<Self::Item, Self::Error> {
		if self.reconsume_now {
			self.reconsume_now = false;
			return Ok(self.current().expect("Le jeton actuel"));
		}

		while let Ok(codepoint) = self.input.consume_next() {
			match codepoint {
				| CodePoint::EOF => break,

				// Saut de ligne
				// Espace blanc
				| codepoint
					if codepoint.is_newline() || codepoint.is_whitespace() =>
				{
					if codepoint.is('\n') {
						self.location.increment_line();
						self.location.reset_column();
					} else if !codepoint.is('\r') {
						self.location.increment_column();
					} else if codepoint.is('\t') {
						self.location.increment_column_by(8);
					}

					continue;
				}

				// chaîne de caractères
				| CodePoint::QUOTATION_MARK
				| CodePoint::APOSTROPHE
				| CodePoint::GRAVE_ACCENT => {
					self.input.reconsume_current();
					let output = self.consume_string()?;
					self.location.increment_column_by(output.location.column);

					return Ok(LexerToken::new(
						TokenKind::Literal(Literal::String(output)),
						self.location,
					));
				}

				// Est un point de code identifier
				| codepoint if codepoint.is_ident() => {
					self.input.reconsume_current();

					let location = self.location;

					let identifier = match self.consume_ident_sequence() {
						| Ok(output) => {
							self.location
								.increment_column_by(output.location.column);
							TokenKind::Identifier(output)
						}
						| Err(IdentifierError::IsKeyword { found }) => {
							TokenKind::Keyword(found)
						}
					};

					return Ok(LexerToken::new(identifier, location));
				}

				// Est un point de code délimiteur
				| _ => {
					self.input.reconsume_current();

					let delimiter = LexicalDelimiter::new(&mut self.input);
					let (output, n) = delimiter.parse()?;
					let location = self.location;
					self.location.increment_column_by(n);
					for _ in 0..n {
						let _ = self.input.consume_next();
					}

					return Ok(LexerToken::new(
						TokenKind::Delimiter(output.delimiter),
						location,
					));
				}
			}
		}

		let token = LexerToken::new(TokenKind::EOF, self.location);
		self.current_token = Some(token.clone());
		Ok(token)
	}

	fn peek_next(&mut self) -> Result<Self::Item, Self::Error> {
		if self.reconsume_now {
			return self.current().ok_or(Self::Error::EOS);
		}

		let token = self.consume_next();
		self.reconsume_current();
		token
	}

	fn reconsume_current(&mut self) {
		self.reconsume_now = true;
	}

	fn peek_n_next(
		&mut self,
		n: usize,
	) -> Vec<Result<Self::Item, Self::Error>> {
		Vec::with_capacity(n)
	}
}

impl StreamIteratorItem for LexerToken {
	type Kind = TokenKind;

	fn eof() -> Self::Kind {
		Self::Kind::EOF
	}

	fn ignore() -> Self::Kind {
		Self::Kind::Ignore
	}

	fn kind(&self) -> &Self::Kind {
		&self.kind
	}
}

impl<C, U> TokenizerAlgorithms for Tokenizer<C, LexerToken, LexicalError>
where
	C: StreamIterator<Item = CodePoint<U>>,
	U: CodePointInterface,
{
	fn consume_ident_sequence(
		&mut self,
	) -> Result<IdentifierOutput, IdentifierError> {
		let identifier = LexicalIdentifier::new(&mut self.input);
		identifier.parse()
	}

	fn consume_string(&mut self) -> Result<StringOutput, StringError> {
		let string = LexicalString::new(&mut self.input);
		string.parse()
	}
}

impl<'a, I, U> Input for LexicalDelimiter<'a, I>
where
	I: StreamIterator<Item = CodePoint<U>>,
	U: CodePointInterface,
{
	type Output = Result<(DelimiterOutput, usize), DelimiterError>;
	type State = ();

	fn parse(mut self) -> Self::Output {
		let a1 = self.input.peek_next();
		let d1 = Delimiter::try_from(a1.unwrap());

		d1.map(|d1| {
			self.location.increment_column_by(1);
			(
				DelimiterOutput {
					delimiter: d1,
					location: self.location,
				},
				1,
			)
		})
	}
}

impl fmt::Display for TokenKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"<{}-token>",
			match self {
				| Self::Identifier(_) => "ident".to_owned(),
				| Self::Keyword(keyword) => format!("keyword-{}", keyword),
				| Self::Literal(literal) => format!("literal-{}", literal),
				| Self::Delimiter(delim) => delim.to_string(),
				| Self::EOF => "EOF".to_owned(),
				| Self::Ignore => "IGNORE".to_owned(),
			}
		)
	}
}

impl ops::Deref for LexerToken {
	type Target = TokenKind;

	fn deref(&self) -> &Self::Target {
		self.kind()
	}
}
