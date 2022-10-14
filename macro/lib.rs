/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Macro derivable `Env`
mod env;
/// Macro attribute `setup`
mod setup;

use proc_macro::TokenStream;
use syn::__private::{quote::quote, ToTokens, TokenStream2};

use self::{env::Analyzer as EnvAnalyzer, setup::Analyzer as SetupAnalyzer};

/// Attribut `setup`. Déclare la fonction principale `main`.
// Utilisation de l'attribut.
//
// ```rust
// 	#[phisyrc::setup]
// 	fn main() { ... }
// ```
//
// Premier paramètre
// ```rust
// 	#[derive(clap::Parser)] struct app_cli { ... }
//
// 	#[phisyrc::setup]
// 	fn main(args: app_cli) { dbg!(&args); }
//
// 	#[phisyrc::setup(logger)]
// 	fn main(args: app_cli) { ... }
// ```
//
// Second paramètre
//
// ```rust
// 	#[derive(clap::Parser)] struct app_cli { ... }
// 	#[derive(Env)] struct app_env { ... }
//
// 	#[phisyrc::setup]
// 	fn main(args: app_cli, env: app_env) { dbg!(&args, &env); }
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

/// Derive `Env`. Déclare une structure de variables d'environnement.
//
// ```rust
// 	#[derive(phisyrc::Env)]
// 	struct app_env {
// 		#[var(key="PHISYRC_NICK", default="PhiSyX")]
// 		nick: String,
// 	}
// ```
#[proc_macro_derive(Env, attributes(var, default))]
pub fn env_trait_derive(input: TokenStream) -> proc_macro::TokenStream {
	let struct_input = syn::parse_macro_input!(input as syn::ItemStruct);
	let analyzer = EnvAnalyzer::new(struct_input);
	match analyzer.build() {
		| Ok(ok) => ok,
		| Err(err) => err.compile_error(),
	}
}

pub(crate) fn field_name(field: &syn::Field) -> String {
	token_to_string(&field.ident)
}

pub(crate) fn token_to_string<T>(tokens: &T) -> String
where
	T: ToTokens,
{
	quote!(#tokens).to_string()
}

pub(crate) fn token_upper(field: &syn::Field) -> TokenStream2 {
	let field = field.ident.as_ref().unwrap().to_string().to_uppercase();
	quote! { #field }
}
