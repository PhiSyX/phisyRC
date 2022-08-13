/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod env;
mod setup;

use proc_macro::TokenStream;

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
pub fn setup(attrs: TokenStream, input: TokenStream) -> TokenStream {
	let attrs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
	let function_input = syn::parse_macro_input!(input as syn::ItemFn);
	let analyzer = setup::SetupAnalyzer::new(function_input, attrs);
	match analyzer.build() {
		| Ok(ok) => ok,
		| Err(err) => err.compile_error(),
	}
}

// Utilisation:
//
// ```rust
//    #[derive(phisyrc::Env)]
//     struct MyEnv {
//        #[var(key="PHISYRC_NICK", default="PhiSyX")]
//        nick: String,
//     }
// ```
#[proc_macro_derive(Env, attributes(var, default))]
pub fn env_trait_derive(input: TokenStream) -> proc_macro::TokenStream {
	let struct_input = syn::parse_macro_input!(input as syn::ItemStruct);
	let analyzer = env::EnvAnalyzer::new(struct_input);
	match analyzer.parse() {
		| Ok(ok) => ok,
		| Err(err) => err.compile_error(),
	}
}
