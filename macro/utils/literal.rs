/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use syn::Lit;

/// Récupère une chaîne de caractère littérale
pub fn get_str(lit: &Lit) -> Option<String> {
	if let Lit::Str(s) = lit {
		return Some(s.value());
	}
	None
}
