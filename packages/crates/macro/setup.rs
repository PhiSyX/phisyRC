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
	punctuated::Punctuated,
	spanned::Spanned,
	token::{self},
	FnArg, Ident, Lit, Meta, NestedMeta, Pat, PatReference, PatTuple, Path,
};

use crate::utils::generic;

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
	/// Le premier argument n'est pas valide.
	FirstArgumentInvalid(Span),
	/// Le second argument n'est pas valide.
	SecondArgumentInvalid(Span),
	/// La clause where est obligatoire lorsqu'un générique est défini.
	MissingWhereClause(Span),
	/// La clause where ne contient pas la générique en question.
	WhereClauseName(&'a syn::Ident, Span),
	/// La générique Async est manquante.
	MissingAsyncGeneric(Span),
	/// La générique Context est manquante.
	MissingContextGeneric(Span),
	/// Trop d'arguments passés à la fonction principale.
	TooManyArguments(usize, Span),
	/// Le jeton n'est pas attendu.
	Unexpected(Span),
	/// L'attribut est manquant dans la liste des attributs supportés :
	/// [Analyzer::SUPPORT_ATTRIBUTES]
	UnknownAttribute(&'a syn::Ident, Span),
}

#[derive(Copy, Clone)]
#[derive(Default)]
enum AsyncAttribute {
	/// Pour l'utilisation de l'attribut `#[actix_web::main]`
	ActixWeb,
	/// Pour l'utilisation de l'attribut `#[tokio::main]`
	#[default]
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
			| 1 => self.build_main_fn(|this| this.main_fn_with_one_arg()),
			| 2 => self.build_main_fn(|this| this.main_fn_with_two_args()),
			| n => return Err(Error::TooManyArguments(n, self.input.span())),
		};

		output.map(|tok| tok.into())
	}
}

impl<'a> Error<'a> {
	/// Erreur de compilation.
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
	/// Les attributs supportés.
	const SUPPORT_ATTRIBUTES: [&str; 3] = ["config", "logger", "database"];
	/// Le nombre d'argument attendu dans la fonction principale main.
	const TOTAL_ARGUMENTS_EXPECTED: usize = 2;

	/// Construit la fonction principale.
	fn build_main_fn<'a>(
		&'a self,
		setup_fn: fn(&'a Self) -> Result<'a, TokenStream2>,
	) -> Result<'a, TokenStream2> {
		let maybe_attrs = self.attrs.iter().map(|nested_meta| {
			if let NestedMeta::Meta(meta) = nested_meta {
				if let Meta::Path(path) = meta {
					return self.handle_attribute(
						meta,
						path,
						Default::default(),
					);
				}

				if let Meta::NameValue(nv) = meta {
					if let Lit::Str(lit_str) = &nv.lit {
						return self.handle_attribute(
							meta,
							&nv.path,
							lit_str.value(),
						);
					}
				}
			}

			Err(Error::Unexpected(nested_meta.span()))
		});

		let setup_by_attrs: Vec<_> = maybe_attrs.collect::<Result<_>>()?;

		let fn_attrs = &self.input.attrs;
		let maybe_asyncness = self.input.sig.asyncness;
		let output_type = &self.input.sig.output;
		let body_block = &self.input.block;
		let setup = setup_fn(self)?;

		let async_tokens = if let Some(maybe_aa) = self.maybe_asyncness() {
			// NOTE(phisyx): cette générique n'est pas obligatoire.
			if let Err(Error::MissingAsyncGeneric(_)) = &maybe_aa {
				quote! {
					#[tokio::main]
				}
			} else {
				match maybe_aa? {
					| AsyncAttribute::ActixWeb => quote! {
						#[actix_web::main]
					},
					| AsyncAttribute::Tokio => quote! {
						#[tokio::main]
					},
				}
			}
		} else {
			quote! {}
		};

		let has_ctx_generic = if let Some(maybe_path) = self.maybe_ctx_generic()
		{
			// NOTE(phisyx): cette générique n'est pas obligatoire.
			if let Err(Error::MissingContextGeneric(_)) = &maybe_path {
				quote! {}
			} else {
				let generic_mpsc = maybe_path?;
				quote! {
					// NOTE(phisyx): on peut laisser choisir d'où provient le `mpsc`.
					let (ctx, mut crx) = #generic_mpsc::mpsc();
				}
			}
		} else {
			return Err(Error::MissingContextGeneric(
				self.input.sig.generics.span(),
			));
		};

		Ok(quote! {
			#(#fn_attrs)*
			#async_tokens
			#maybe_asyncness fn main() #output_type {
				#has_ctx_generic
				#setup
				#(#setup_by_attrs)*
				#body_block
			}
		})
	}

	fn handle_attribute<'a>(
		&'a self,
		meta: &'a Meta,
		path: &'a Path,
		mut arg_lit: String,
	) -> Result<'a, TokenStream2> {
		let attribute_name = path
			.get_ident()
			.expect("Devrait être un identifiant valide");

		if !Self::SUPPORT_ATTRIBUTES
			.contains(&attribute_name.to_string().as_str())
		{
			return Err(Error::UnknownAttribute(attribute_name, meta.span()));
		}

		// Génère un tuple contenant les arguments passés dans la fonction
		// principale main :
		//
		//     1. ()
		//     2. (cli)
		//     3. (cli, env)
		let arguments_from_main_fn = {
			let mut list = Punctuated::new();
			// CLI
			if let Some(item) = self.get_first_arg_pat() {
				list.push(item);
			}
			// ENV
			if let Some(item) = self.get_last_arg_pat() {
				list.push(item);
			}
			PatTuple {
				attrs: Default::default(),
				paren_token: token::Paren(meta.span()),
				elems: list,
			}
		};

		let total_arguments_from_main_fn = arguments_from_main_fn.elems.len();
		let arguments_from_main_fn = {
			if total_arguments_from_main_fn == 0 {
				None
			} else {
				Some(arguments_from_main_fn)
			}
		};

		// Génère un identifiant de fonction selon si l'on est en asynchrone et
		// selon les arguments de l'attribut en question.
		//
		// Exemple:
		//
		//    - future_logger_tui (async)
		//    - logger_stdout
		let attribute_name_fn = {
			if !arg_lit.is_empty() {
				arg_lit = format!("_{arg_lit}");
			}
			if self.input.sig.asyncness.is_some() {
				if total_arguments_from_main_fn > 0 {
					Ident::new(
						&format!("future_{attribute_name}{arg_lit}_{total_arguments_from_main_fn}"),
						attribute_name.span(),
					)
				} else {
					Ident::new(
						&format!("future_{attribute_name}{arg_lit}"),
						attribute_name.span(),
					)
				}
			} else if total_arguments_from_main_fn > 0 {
				Ident::new(
  				&format!("{attribute_name}{arg_lit}_{total_arguments_from_main_fn}"),
  				attribute_name.span(),
  			)
			} else {
				Ident::new(
					&format!("{attribute_name}{arg_lit}"),
					attribute_name.span(),
				)
			}
		};

		// Génère un identifiant de variable pour stocker le résultat de la
		// fonction générée ci-haut (attribute_name_fn).
		let result_attribute_from_setup_fn = Ident::new(
			&format!("maybe_{attribute_name}"),
			attribute_name.span(),
		);

		// Génère un identifiant de variable qui correspond au contexte
		// d'application.
		let app_ctx = if let Some(maybe_ctx) = self.maybe_ctx_generic() {
			if let Err(Error::MissingContextGeneric(_)) = &maybe_ctx {
				None
			} else {
				let ctx = maybe_ctx?;
				Some(Ident::new("ctx", ctx.span()))
			}
		} else {
			None
		};

		let call = {
			let optional_app_ctx =
				if app_ctx.is_some() && total_arguments_from_main_fn > 0 {
					Some(quote! {, #app_ctx.clone() })
				} else if app_ctx.is_some() {
					Some(quote! { #app_ctx.clone() })
				} else {
					None
				};

			let list_params = quote! {
				( #arguments_from_main_fn #optional_app_ctx )
			};

			if self.input.sig.asyncness.is_some() {
				quote! {
					setup::#attribute_name_fn #list_params .await
				}
			} else {
				quote! {
					setup::#attribute_name_fn #list_params
				}
			}
		};

		Ok(quote! {
			#[allow(unused_variables, unused_parens)]
			let #result_attribute_from_setup_fn = #call;

			if let Err(err) = #result_attribute_from_setup_fn {
				use terminal::format::color::Interface;
				eprintln!();
				eprintln!(
					"{}: #[phisyrc::setup({})]: quelque chose s'est mal passée -- {}",
					"runtime error".red(),
					stringify!(#attribute_name),
					err
				);
				eprintln!();
				std::process::exit(1);
			}
		})
	}

	fn get_first_arg_pat(&self) -> Option<Pat> {
		let inputs = &self.input.sig.inputs;

		if inputs.is_empty() {
			return None;
		}

		let first_argument = inputs.first().and_then(|arg| match arg {
			| FnArg::Typed(paty) => Some(paty.pat.clone()),
			| FnArg::Receiver(_) => None,
		});

		first_argument.map(|boxed_pat| match &*boxed_pat {
			| Pat::Ident(_) => Pat::Reference(PatReference {
				attrs: Default::default(),
				and_token: syn::token::And(boxed_pat.span()),
				mutability: Default::default(),
				pat: boxed_pat,
			}),
			| _ => unreachable!("#[phisyrc::setup]: get_first_arg_pat"),
		})
	}

	fn get_last_arg_pat(&self) -> Option<Pat> {
		let inputs = &self.input.sig.inputs;

		if inputs.len() != Self::TOTAL_ARGUMENTS_EXPECTED {
			return None;
		}

		let last_argument = inputs.last().and_then(|arg| match arg {
			| FnArg::Typed(paty) => Some(paty.pat.clone()),
			| FnArg::Receiver(_) => None,
		});

		last_argument.map(|boxed_pat| match &*boxed_pat {
			| Pat::Ident(_) => Pat::Reference(PatReference {
				attrs: Default::default(),
				and_token: syn::token::And(boxed_pat.span()),
				mutability: Default::default(),
				pat: boxed_pat,
			}),
			| _ => unreachable!("#[phisyrc::setup]: get_last_arg_pat"),
		})
	}

	fn maybe_ctx_generic(&self) -> Option<Result<&Path>> {
		let fn_gen = &self.input.sig.generics;

		generic::find_generic_and_clause(fn_gen, &"Context")
			.map_err(|err| match err {
				| generic::Error::MissingGeneric(span) => {
					Error::MissingContextGeneric(span)
				}
				| generic::Error::MissingWhereClause(Some(ident), span) => {
					Error::WhereClauseName(ident, span)
				}
				| generic::Error::MissingWhereClause(None, span) => {
					Error::MissingWhereClause(span)
				}
			})
			.map(Into::into)
			.transpose()
	}

	fn maybe_asyncness(&self) -> Option<Result<AsyncAttribute>> {
		self.input.sig.asyncness?;

		let fn_gen = &self.input.sig.generics;
		if fn_gen.params.is_empty() {
			return Some(Ok(AsyncAttribute::Tokio));
		}

		generic::find_generic_and_clause(fn_gen, &"Async")
			.map_err(|err| match err {
				| generic::Error::MissingGeneric(span) => {
					Error::MissingAsyncGeneric(span)
				}
				| generic::Error::MissingWhereClause(Some(ident), span) => {
					Error::WhereClauseName(ident, span)
				}
				| generic::Error::MissingWhereClause(None, span) => {
					Error::MissingWhereClause(span)
				}
			})
			.map(|p| p.get_ident().map(AsyncAttribute::from))
			.transpose()
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

	/// Utilisé lorsque la fonction principale compte un seul argument
	///
	/// NOTE(phisyx): Le premier argument est la CLI. La CLI DOIT toujours
	/// contenir la fonction/méthode `arguments`.
	fn main_fn_with_one_arg(&self) -> Result<TokenStream2> {
		let inputs = &self.input.sig.inputs;

		let first_argument = inputs
			.first()
			.filter(|argument| matches!(argument, FnArg::Typed(_)))
			.and_then(|argument| match argument {
				| FnArg::Typed(t) => Some((&t.pat, &t.ty)),
				| FnArg::Receiver(_) => None,
			});

		if first_argument.is_none() {
			return Err(Error::FirstArgumentInvalid(self.input.span()));
		}

		let (pat, ty) = first_argument.unwrap();

		Ok(quote! {
			let #pat = #ty::arguments(); // <- voir la NOTE ci-haut.
		})
	}

	/// Utilisé lorsque la fonction principale compte deux arguments
	///
	/// NOTE(phisyx): Le second argument a les variables d'environnement.
	fn main_fn_with_two_args(&self) -> Result<TokenStream2> {
		// CLI
		let mut for_one_arg_tokens = self.main_fn_with_one_arg()?;

		// Continue...

		let inputs = &self.input.sig.inputs;

		let (pat, ty) = inputs
			.last()
			.filter(|arg| matches!(arg, FnArg::Typed(_)))
			.and_then(|arg| match arg {
				| FnArg::Typed(typed) => Some((&typed.pat, &typed.ty)),
				| _ => None,
			})
			.ok_or_else(|| Error::SecondArgumentInvalid(self.input.span()))?;

		let for_two_args_tokens = quote! {
			use env::Interface;

			let #pat = #ty::setup(if cfg!(debug_assertions) {
				".env.local"
			} else {
				".env"
			}).expect(
				"Erreur lors de la récupération des variables d'environnement"
			);
		};

		for_one_arg_tokens.extend(for_two_args_tokens);
		Ok(for_one_arg_tokens)
	}
}

impl<'a> Error<'a> {
	fn span(self) -> Span {
		match self {
			| Self::IsNotMainFunction(span)
			| Self::FirstArgumentInvalid(span)
			| Self::SecondArgumentInvalid(span)
			| Self::MissingWhereClause(span)
			| Self::WhereClauseName(_, span)
			| Self::MissingAsyncGeneric(span)
			| Self::MissingContextGeneric(span)
			| Self::TooManyArguments(_, span)
			| Self::Unexpected(span)
			| Self::UnknownAttribute(_, span) => span,
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<&syn::Ident> for AsyncAttribute {
	fn from(ident: &syn::Ident) -> Self {
		if ident.eq("actix_web") {
			return Self::ActixWeb;
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
			| Self::FirstArgumentInvalid(_) => {
				"le premier argument de la fonction principale est invalide."
					.to_owned()
			}
			| Self::SecondArgumentInvalid(_) => {
				"le second argument de la fonction principale est invalide."
					.to_owned()
			}
			| Self::MissingWhereClause(_) => {
				"Une clause `where` est attendue.".to_owned()
			}
			| Self::WhereClauseName(ident, _) => {
				format!(
					"La clause where ne contient pas la générique « {} ».",
					ident
				)
			}
			| Self::MissingAsyncGeneric(_) => {
				"La générique 'Async' est manquante.".to_owned()
			}
			| Self::MissingContextGeneric(_) => {
				"La générique 'Context' est manquante.".to_owned()
			}
			| Self::TooManyArguments(n, _) => {
				format!(
					"la fonction principale ne peut avoir plus de « {} » \
					arguments maximum (le nombre d'arguments actuel est de « {} »).",
					Analyzer::TOTAL_ARGUMENTS_EXPECTED,
					n
				)
			}
			| Self::Unexpected(_) => "seuls des identifiants sont attendus. \
				 Un exemple valide: #[phisyrc::setup(ident1, ident2)]"
				.to_owned(),
			| Self::UnknownAttribute(attr_ident, _) => {
				format!("l'attribut `{attr_ident}` n'est pas reconnu.")
			}
		};
		write!(f, "#[phisyrc::setup]: {reason}")
	}
}
