/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Deserializer};

// --------- //
// Structure //
// --------- //

pub(super) struct SerdeValidation;

// -------------- //
// Implémentation //
// -------------- //

impl SerdeValidation {
	pub(super) fn string_not_empty<'de, D>(de: D) -> Result<String, D::Error>
	where
		D: Deserializer<'de>,
	{
		use serde::de::Error;
		String::deserialize(de)
			.ok()
			.filter(|s| !s.trim().is_empty())
			.ok_or_else(|| D::Error::custom("La chaîne de caractère est vide."))
	}
}
