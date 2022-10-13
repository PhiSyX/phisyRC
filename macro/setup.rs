/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;

use proc_macro::TokenStream;
use syn::{
	__private::{
		quote::{quote, quote_spanned},
		Span, TokenStream2,
	},
	spanned::Spanned,
	FnArg, GenericParam, TypeParamBound, WherePredicate,
};

// ---- //
// Type //
// ---- //

type Result<'a, T> = core::result::Result<T, Error<'a>>;

type Attrs = syn::AttributeArgs;
type Input = syn::ItemFn;

// --------- //
// Structure //
// --------- //

pub(super) struct Analyzer {
	attrs: Attrs,
	input: Input,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub(super) enum Error<'a> {
	/// La macro n'est pas appliquée sur la fonction principale `main`.
	IsNotMainFunction(Span),
	/// La clause where est obligatoire lorsqu'un générique est défini.
	MissingWhereClause(Span),
	/// Trop d'arguments passés à la fonction principale.
	TooManyArguments(usize, Span),
	/// Le jeton n'est pas attendu.
	Unexpected(Span),
}

	/// Pour l'utilisation de l'attribut tokio::main
	Tokio,
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl Analyzer {
	/// Initialise la structure
	pub(super) fn new(input: Input, attrs: Attrs) -> Self {
		Self { input, attrs }
	}

	/// Construit la fonction principale en fonction du nombre de paramètres.
	pub(super) fn build(&self) -> Result<'_, TokenStream> {
		if !self.is_main_fn() {
			return Err(Error::IsNotMainFunction(self.input.span()));
		}

		let inputs = &self.input.sig.inputs;
		let output = match inputs.len() {
			| 0 => self.build_main_fn(|this| this.main_fn_with_zeroed_arg()),
			| n => return Err(Error::TooManyArguments(n, self.input.span())),
		};

		output.map(|tok| tok.into())
	}
}

impl<'a> Error<'a> {
	pub(super) fn compile_error(self) -> TokenStream {
		let error_s = self.to_string();
		let tokens = quote_spanned! {
			self.span() => compile_error!(#error_s);
		};
		TokenStream::from(tokens)
	}
}

// -------------- //
// Implémentation // -> Privé
// -------------- //

impl Analyzer {
	const TOTAL_ARGUMENTS_EXPECTED: usize = 0;

	/// Construit la fonction principale.
	fn build_main_fn<'a>(
		&'a self,
		setup_fn: fn(&'a Self) -> Result<'a, TokenStream2>,
	) -> Result<'a, TokenStream2> {
		let fn_attrs = &self.input.attrs;
		let maybe_asyncness = self.input.sig.asyncness;
		let output_type = &self.input.sig.output;
		let block = &self.input.block;
		let setup = setup_fn(self)?;

		let async_tokens = if let Some(maybe_aa) = self.maybe_asyncness() {
			match maybe_aa? {
				| AsyncAttribute::Tokio => quote! {
					#[tokio::main]
				},
			}
		} else {
			quote! {}
		};

		Ok(quote! {
			#(#fn_attrs)*
			#async_tokens
			#maybe_asyncness fn main() #output_type {
				#setup
				#block
			}
		})
	}

	fn maybe_asyncness(&self) -> Option<Result<AsyncAttribute>> {
		self.input.sig.asyncness?;

		let fn_gen = &self.input.sig.generics;
		if fn_gen.params.is_empty() {
			return Some(Ok(AsyncAttribute::Tokio));
		}

		let first_param =
			if let Some(GenericParam::Type(ty_param)) = fn_gen.params.first() {
				Some(ty_param)
			} else {
				None
			}?;

		// <Async>
		// if !first_param.ident.eq("Async") {
		// 	return Some(Ok(AsyncAttribute::Tokio));
		// }

		let maybe_where_clause = fn_gen.where_clause.as_ref();
		if maybe_where_clause.is_none() {
			panic!("{}", Error::MissingWhereClause(first_param.span()));
		}
		let where_clause = maybe_where_clause.unwrap();

		// where
		//   Async: path
		if let Some(WherePredicate::Type(predicate_ty)) =
			where_clause.predicates.first()
		{
			let has_same_clause = match &predicate_ty.bounded_ty {
				| syn::Type::Path(ty_path)
					if ty_path.path.is_ident(&first_param.ident) =>
				{
					true
				}
				| _ => false,
			};

			if !has_same_clause {
				return None;
			}

			let ty_param = predicate_ty.bounds.first()?;
			if let TypeParamBound::Trait(bound) = ty_param {
				return Some(Ok(AsyncAttribute::from(bound.path.get_ident()?)));
			}
		}

		Some(Ok(AsyncAttribute::Tokio))
	}

	/// Vérifie que l'identifiant de la signature de la fonction est la fonction
	/// principale main.
	fn is_main_fn(&self) -> bool {
		self.input.sig.ident.eq("main")
	}

	/// Utilisé lorsque la fonction principale ne compte aucun argument.
	fn main_fn_with_zeroed_arg(&self) -> Result<TokenStream2> {
		Ok(quote! {})
	}
}

impl<'a> Error<'a> {
	fn span(self) -> Span {
		match self {
			| Self::IsNotMainFunction(span)
			| Self::MissingWhereClause(span)
			| Self::TooManyArguments(_, span)
			| Self::Unexpected(span)
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<&syn::Ident> for AsyncAttribute {
	fn from(ident: &syn::Ident) -> Self {
		if ident.eq("tokio") {
			return Self::Tokio;
		}
		Self::Tokio
	}
}

impl fmt::Display for Error<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let reason = match self {
			| Self::IsNotMainFunction(_) => {
				"ne peut être utilisé que sur la fonction principale: \
				 fn main() {}"
					.to_owned()
			}
			| Self::MissingWhereClause(_) => {
				"Une clause `where` est attendue.".to_owned()
			}
			| Self::TooManyArguments(n, _) => {
				format!(
					"la fonction principale ne peut avoir plus de {} \
					arguments maximum (le nombre d'arguments actuel est de {}).",
					Analyzer::TOTAL_ARGUMENTS_EXPECTED,
					n
				)
			}
			| Self::Unexpected(_) => "seuls des identifiants sont attendus. \
				 Un exemple valide: #[phisyrc::setup(ident1, ident2)]"
				.to_owned(),
		};
		write!(f, "#[phisyrc::setup]: {reason}")
	}
}
