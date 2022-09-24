/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use ::serde::{de::Error, Deserialize, Deserializer};

// --------- //
// Structure //
// --------- //

pub struct SerdeValidation;

// -------------- //
// Implémentation //
// -------------- //

impl SerdeValidation {
	pub fn string_not_empty<'de, D>(de: D) -> Result<String, D::Error>
	where
		D: Deserializer<'de>,
	{
		String::deserialize(de)
			.ok()
			.filter(|s| !s.trim().is_empty())
			.ok_or_else(|| D::Error::custom("La chaîne de caractère est vide."))
	}
}
