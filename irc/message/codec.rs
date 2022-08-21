/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::ops::{Deref, DerefMut};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Framed, LinesCodec};

// --------- //
// Structure //
// --------- //

pub struct IrcCodec<S>(Framed<S, LinesCodec>);

// -------------- //
// Implémentation //
// -------------- //

impl<S> IrcCodec<S>
where
	S: AsyncRead + AsyncWrite,
{
	const MAX_LINE_LENGTH: usize = 20_000;

	pub fn new(stream: S) -> Self {
		Self(Framed::new(
			stream,
			LinesCodec::new_with_max_length(Self::MAX_LINE_LENGTH),
		))
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<S> Deref for IrcCodec<S> {
	type Target = Framed<S, LinesCodec>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<S> DerefMut for IrcCodec<S> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
