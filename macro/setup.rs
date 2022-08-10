/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use syn::{
	__private::{
		quote::{quote, quote_spanned},
		TokenStream2,
	},
	spanned::Spanned,
};

/// Construit la fonction principale.
pub(super) fn build_fn(
	input: &syn::ItemFn,
	setup_fn: fn(&syn::ItemFn) -> TokenStream2,
) -> TokenStream2 {
	let attributes = &input.attrs;
	let maybe_asyncness = input.sig.asyncness;
	let output_type = &input.sig.output;
	let block = &input.block;
	let setup = setup_fn(input);

	quote! {
	#(#attributes)*
	#[tokio::main]
	#maybe_asyncness fn main() #output_type {
		#setup

		#block
	}
	}
}

/// Quand il n'y a aucun argument, on ne fait rien de particulier.
pub(super) fn function_with_zeroed_arg(_input: &syn::ItemFn) -> TokenStream2 {
	quote! {}
}

/// Le premier argument est la CLI. La CLI DOIT toujours contenir la fonction
/// `arguments`.
pub(super) fn function_with_one_arg(input: &syn::ItemFn) -> TokenStream2 {
	let inputs = &input.sig.inputs;

	let first_arg = inputs
		.first()
		.filter(|arg| matches!(arg, syn::FnArg::Typed(_)))
		.map(|arg| match arg {
			| syn::FnArg::Typed(typed) => Some((&typed.pat, &typed.ty)),
			| _ => None,
		})
		.expect("Le premier argument");

	if first_arg.is_none() {
		return quote_spanned! {
			input.span() => compile_error!(
				"#[phisyrc::setup]: argument incorrect (#1)."
			)
		};
	}

	let (first_arg_pat, first_arg_ty) = first_arg.unwrap();

	quote! {
		let #first_arg_pat = #first_arg_ty::arguments();
	}
}
