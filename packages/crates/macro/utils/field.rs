/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use syn::{
	Fields,
	__private::{quote::quote, ToTokens, TokenStream2},
};

/// Le nom d'un champ (son identifiant)
pub(crate) fn name(field: &syn::Field) -> String {
	let tokens = field.ident.to_token_stream();
	tokens.to_string()
}

pub(crate) fn token_upper(field: &syn::Field) -> TokenStream2 {
	let field = field.ident.as_ref().unwrap().to_string().to_uppercase();
	quote! { #field }
}

/// Vérifie que la structure est une structure de champs nommés.
pub(crate) fn is_structure_of_fields_named(fields: &Fields) -> bool {
	matches!(fields, syn::Fields::Named(_))
}
