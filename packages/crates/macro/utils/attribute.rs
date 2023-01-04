/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use syn::{punctuated::Punctuated, token, Attribute, Field, Lit, NestedMeta};

/// Récupère une propriété (prop) d'une liste d'attributs.
///
/// -> #[attr( prop = "value", prop2 = "value2" )]
pub fn get_prop_in_list(
	list: &Punctuated<NestedMeta, token::Comma>,
	prop: impl AsRef<str>,
) -> Option<&Lit> {
	list.iter().find_map(|nested_meta| {
		if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) =
			nested_meta
		{
			name_value
				.path
				.is_ident(prop.as_ref())
				.then_some(&name_value.lit)
		} else {
			None
		}
	})
}

/// Cherche l'attribut passé en argument parmi la liste des attributs d'un
/// champ.
pub fn find_field(
	field: &Field,
	attr_name: impl AsRef<str>,
) -> Option<&Attribute> {
	field
		.attrs
		.iter()
		.find(|attr| attr.path.is_ident(attr_name.as_ref()))
}
