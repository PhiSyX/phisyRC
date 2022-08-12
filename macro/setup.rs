/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

use proc_macro::TokenStream;
use syn::{
	__private::{
		quote::{quote, quote_spanned},
		Span, TokenStream2,
	},
	spanned::Spanned,
};

pub(super) struct SetupAnalyzer {
	function_input: syn::ItemFn,
}

pub(super) enum SetupAnalyzerError {
	IsNotMainFunction(Span),
	FirstArgumentInvalid(Span),
	SecondArgumentInvalid(Span),
	TooManyArguments(Span, usize),
}

impl SetupAnalyzer {
	const TOTAL_ARGS_EXPECTED: usize = 2;

	pub(super) fn new(function_input: syn::ItemFn) -> Self {
		Self { function_input }
	}

	pub(super) fn build(self) -> Result<TokenStream, SetupAnalyzerError> {
		if !self.is_main_function() {
			return Err(SetupAnalyzerError::IsNotMainFunction(
				self.function_input.span(),
			));
		}

		let inputs = &self.function_input.sig.inputs;

		let output = match inputs.len() {
			| 0 => self.build_main_fn(|this| this.main_fn_with_zeroed_arg()),
			| 1 => self.build_main_fn(|this| this.main_fn_with_one_arg()),
			| 2 => self.build_main_fn(|this| this.main_fn_with_two_args()),
			| _ => {
				return Err(SetupAnalyzerError::TooManyArguments(
					self.function_input.span(),
					inputs.len(),
				));
			}
		};

		output.map(|tok| tok.into())
	}

	/// Construit la fonction principale.
	fn build_main_fn(
		&self,
		setup_fn: fn(&Self) -> Result<TokenStream2, SetupAnalyzerError>,
	) -> Result<TokenStream2, SetupAnalyzerError> {
		let attrs = &self.function_input.attrs;
		let maybe_asyncness = self.function_input.sig.asyncness;
		let output_type = &self.function_input.sig.output;
		let block = &self.function_input.block;
		let setup = setup_fn(self)?;

		Ok(quote! {
		#(#attrs)*
		#[tokio::main]
		#maybe_asyncness fn main() #output_type {
			#setup
			#block
		}
		})
	}

	fn is_main_function(&self) -> bool {
		self.function_input.sig.ident.eq("main")
	}

	// 0
	/// Quand il n'y a aucun argument, on ne fait rien de particulier.
	fn main_fn_with_zeroed_arg(
		&self,
	) -> Result<TokenStream2, SetupAnalyzerError> {
		Ok(quote! {})
	}

	// 1
	/// Le premier argument est la CLI. La CLI DOIT toujours contenir la
	/// fonction `arguments`.
	fn main_fn_with_one_arg(&self) -> Result<TokenStream2, SetupAnalyzerError> {
		let inputs = &self.function_input.sig.inputs;

		let first_arg = inputs
			.first()
			.filter(|arg| matches!(arg, syn::FnArg::Typed(_)))
			.map(|arg| match arg {
				| syn::FnArg::Typed(typed) => Some((&typed.pat, &typed.ty)),
				| _ => None,
			})
			.expect("Le premier argument");

		if first_arg.is_none() {
			return Err(SetupAnalyzerError::FirstArgumentInvalid(
				self.function_input.span(),
			));
		}

		let (first_arg_pat, first_arg_ty) = first_arg.unwrap();

		Ok(quote! {
			let #first_arg_pat = #first_arg_ty::arguments();
		})
	}

	// 2
	/// Le second argument est constitué de variables d'ENV.
	fn main_fn_with_two_args(
		&self,
	) -> Result<TokenStream2, SetupAnalyzerError> {
		let mut for_one_arg_tokens = self.main_fn_with_one_arg()?;

		let inputs = &self.function_input.sig.inputs;
		let last_arg = inputs
			.last()
			.filter(|arg| matches!(arg, syn::FnArg::Typed(_)))
			.map(|arg| match arg {
				| syn::FnArg::Typed(typed) => Some((&typed.pat, &typed.ty)),
				| _ => None,
			})
			.expect("Le dernier argument");

		if last_arg.is_none() {
			return Err(SetupAnalyzerError::SecondArgumentInvalid(
				self.function_input.span(),
			));
		}

		let (last_arg_pat, last_arg_ty) = last_arg.unwrap();

		let for_two_args_tokens = quote! {
			use env::EnvInterface;

			let #last_arg_pat = #last_arg_ty::setup(".env.local") // TODO(phisyx): à changer
				.expect("Erreur lors de la récupération des variables d'environnements");
		};

		for_one_arg_tokens.extend(for_two_args_tokens);
		Ok(for_one_arg_tokens)
	}
}

impl SetupAnalyzerError {
	pub(super) fn span(self) -> Span {
		match self {
			| Self::IsNotMainFunction(span)
			| Self::FirstArgumentInvalid(span)
			| Self::SecondArgumentInvalid(span)
			| Self::TooManyArguments(span, _) => span,
		}
	}

	pub(super) fn compile_error(self) -> TokenStream {
		let err_str = self.to_string();
		let tokens = quote_spanned! {
			self.span() => compile_error!(#err_str);
		};
		TokenStream::from(tokens)
	}
}

impl fmt::Display for SetupAnalyzerError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"#[phisyrc::setup]: {}",
			match self {
				| Self::IsNotMainFunction(_) => {
					"ne peut être utilisé que sur la fonction principale: \
					fn main() {}"
						.to_owned()
				}

				| Self::FirstArgumentInvalid(_) => {
					"le premier argument de la fonction principale est invalide."
						.to_owned()
				}

				| Self::SecondArgumentInvalid(_) => {
					"le second argument de la fonction principale est invalide."
						.to_owned()
				}

				| Self::TooManyArguments(_, len) => {
					format!(
						"la fonction principale ne peut avoir plus de {} \
						arguments maximum (le nombre d'arguments actuel est de {}).",
						SetupAnalyzer::TOTAL_ARGS_EXPECTED,
						len
					)
				}
			}
		)
	}
}
