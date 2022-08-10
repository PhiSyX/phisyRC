/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod setup;

use proc_macro::TokenStream;
use syn::{__private::quote::quote_spanned, spanned::Spanned};

// Utilisation:
// ```rust
//    #[phisyrc::setup]
//    fn main() {}
// ```
//
// ```rust
//    use cli::CLI;
//    #[phisyrc::setup]
//    fn main(args: CLI<...>) {}
// ```
#[proc_macro_attribute]
pub fn setup(_: TokenStream, input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as syn::ItemFn);

	let ident = &input.sig.ident;

	if ident.ne("main") {
		let tokens = quote_spanned! {
			ident.span() =>
				compile_error!(
				   "#[phisyrc::setup] ne peut être utilisé que sur la fonction \
					principale: fn main() {}"
				);
		};

		return TokenStream::from(tokens);
	}

	let inputs = &input.sig.inputs;
	let _total_args_expected: u8 = 1;

	let result = match inputs.len() {
		| 0 => setup::build_fn(&input, setup::function_with_zeroed_arg),
		| 1 => setup::build_fn(&input, setup::function_with_one_arg),
		| _ => {
			quote_spanned! {
				inputs.span() => compile_error!( // voir _total_args_expected
					"[phisyrc::setup]: nombre total de paramètres attendu: 1."
				);
			}
		}
	};

	result.into()
}
