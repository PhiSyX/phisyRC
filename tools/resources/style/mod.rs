/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod lexer;
mod parser;

use lang::lexer::LexicalError;
use lang::stream::TokenStream as TokenStreamBase;

pub use self::{lexer::*, parser::*};

// ---- //
// Type //
// ---- //

pub type TokenStream = TokenStreamBase<LexerToken, LexicalError>;
