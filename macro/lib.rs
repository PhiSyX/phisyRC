/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Macro attribute `setup`
mod setup;

use proc_macro::TokenStream;

use self::setup::Analyzer as SetupAnalyzer;

/// Attribut `setup`. Déclare la fonction principale `main`.
// Utilisation de l'attribut.
//
// ```rust
// 	#[phisyrc::setup]
// 	fn main() { ... }
// ```
// ```
#[proc_macro_attribute]
pub fn setup(attrs: TokenStream, input: TokenStream) -> TokenStream {
	let attrs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
	let input_fn = syn::parse_macro_input!(input as syn::ItemFn);
	let analyzer = SetupAnalyzer::new(input_fn, attrs);
	match analyzer.build() {
		| Ok(r) => r,
		| Err(err) => err.compile_error(),
	}
}
