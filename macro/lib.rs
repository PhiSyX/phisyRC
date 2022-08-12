/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod env;
mod setup;

use proc_macro::TokenStream;
use syn::__private::quote::quote_spanned;

// Utilisation:
//
// ```rust
//    #[derive(phisyrc::Env)]
//     struct MyEnv {
//        #[var(key="PHISYRC_NICK", default="PhiSyX")]
//        nick: String,
//     }
// ```
#[proc_macro_derive(Env, attributes(var))]
pub fn env_trait_derive(input: TokenStream) -> proc_macro::TokenStream {
	let struct_input = syn::parse_macro_input!(input as syn::ItemStruct);
	let analyzer = env::EnvAnalyzer::new(struct_input);
	match analyzer.parse() {
		| Ok(ok) => ok,
		| Err(err) => {
			let err_str = err.to_string();
			let tokens = quote_spanned! {
				err.span() => compile_error!(#err_str);
			};
			TokenStream::from(tokens)
		}
	}
}

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
	let input_function = syn::parse_macro_input!(input as syn::ItemFn);

	let ident = &input_function.sig.ident;

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

	let inputs = &input_function.sig.inputs;
	let total_args_expected: u8 = 2;

	let result = match inputs.len() {
		| 0 => {
			setup::build_fn(&input_function, setup::function_with_zeroed_arg)
		}
		| 1 => setup::build_fn(&input_function, setup::function_with_one_arg),
		| 2 => setup::build_fn(&input_function, setup::function_with_two_args),
		| _ => {
			let err_msg = format!(
				"la fonction principale comporte trop de paramètres ({} / {} max).",
				inputs.len(),
				total_args_expected
			);

			quote_spanned! {
				ident.span() =>
					compile_error!(#err_msg);
			}
		}
	};

	result.into()
}
