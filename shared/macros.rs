/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// ----- //
// Macro //
// ----- //

#[macro_export]
macro_rules! err {
	(
	$(
		$(#[$attr:meta])*
		| $variant:ident ( $err:path ) => $reason:literal
	)*
	) => {
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Error {
	$(
		$(#[$attr])*
		$variant($err)
	),*
}

$(
impl From<$err> for Error {
	fn from(err: $err) -> Self {
		Self::$variant(err)
	}
}
)*

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let err = match self {
			$( | Self::$variant(err) => format!($reason, err) ),*
		};
		write!(f, "{}", err)
	}
}
	};
}
