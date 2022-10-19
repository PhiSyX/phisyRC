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
	FnArg, GenericParam, Ident, Meta, Pat, PatReference, PatTuple, Path, Type,
	TypeParamBound, TypeReference, TypeTuple, WherePredicate,
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
	/// Le premier argument n'est pas valide.
	FirstArgumentInvalid(Span),
	/// Le second argument n'est pas valide.
	SecondArgumentInvalid(Span),
	/// La clause where est obligatoire lorsqu'un générique est défini.
	MissingWhereClause(Span),
	/// Trop d'arguments passés à la fonction principale.
	TooManyArguments(usize, Span),
	/// Le jeton n'est pas attendu.
	Unexpected(Span),
	/// L'attribut est manquant dans la liste des attributs supportés :
	/// [Analyzer::SUPPORT_ATTRIBUTES]
	UnknownAttribute(&'a syn::Ident, Span),
}

enum AsyncAttribute {
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
			| 1 => self.build_main_fn(|this| this.main_fn_with_one_arg()),
			| 2 => self.build_main_fn(|this| this.main_fn_with_two_args()),
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
	const SUPPORT_ATTRIBUTES: [&str; 2] = ["logger", "database"];
	const TOTAL_ARGUMENTS_EXPECTED: usize = 2;

	/// Construit la fonction principale.
	fn build_main_fn<'a>(
		&'a self,
		setup_fn: fn(&'a Self) -> Result<'a, TokenStream2>,
	) -> Result<'a, TokenStream2> {
		let maybe_attrs = self.attrs.iter().map(|meta| match meta {
			| syn::NestedMeta::Meta(meta) => match meta {
				| syn::Meta::Path(path) => {
					self.handle_path(meta, path, Default::default())
				}
				| syn::Meta::NameValue(nv) => match &nv.lit {
					| syn::Lit::Str(lit_str) => {
						self.handle_path(meta, &nv.path, lit_str.value())
					}
					| _ => Err(Error::Unexpected(meta.span())),
				},
				| _ => Err(Error::Unexpected(meta.span())),
			},
			| _ => Err(Error::Unexpected(meta.span())),
		});

		let mut setup_by_attrs = Vec::with_capacity(maybe_attrs.len());
		for attr in maybe_attrs {
			setup_by_attrs.push(attr?);
		}

		let fn_attrs = &self.input.attrs;
		let maybe_asyncness = self.input.sig.asyncness;
		let output_type = &self.input.sig.output;
		let body_block = &self.input.block;
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

		let params_ty = {
			let mut list = Punctuated::new();
			if let Some(i) = self.get_first_arg_ty() {
				list.push(i);
			}
			if let Some(i) = self.get_last_arg_ty() {
				list.push(i);
			}
			TypeTuple {
				paren_token: syn::token::Paren(self.input.span()),
				elems: list,
			}
		};
		Ok(quote! {
			#(#fn_attrs)*
			#async_tokens
			#maybe_asyncness fn main() #output_type {
				#setup
				#(#setup_by_attrs)*
				#body_block
			}

			mod setup {
				use super::*;

				use cli::ProcessEnv;
				use helpers::lang::WildcardMatching;
				use logger::{LoggerType, stdout, tui};
				use database::{DatabaseType};

				pub(super) async fn logger(
					ctx: app::AppContextWriter,
					args: #params_ty,
					ty: impl Into<LoggerType>
				) -> Option<tokio::task::JoinHandle<std::io::Result<()>>> {
					let (cli_args, env_args) = args;

					let level_filter = match &cli_args.options.mode {
						| ProcessEnv::DEVELOPMENT => logger::LevelFilter::Debug,
						| ProcessEnv::PRODUCTION => logger::LevelFilter::Off,
						| ProcessEnv::TEST => logger::LevelFilter::Trace,
					};

					let logger_type = ty.into();

					let debug_filter = env_args.debug_filter.clone();
					let logger_builder = stdout::Logger::builder()
						.with_color()
						.with_level(level_filter)
						.with_timestamp()
						.filter(move |metadata| {
							metadata.target().iswm(&debug_filter)
						});

					if LoggerType::Stdout == logger_type || cli_args.command.is_some() {
						logger_builder.build_stdout()
						.expect(
							"Le logger ne DOIT pas s'initialiser plusieurs fois."
						);
					} else if LoggerType::Tui == logger_type {
						return Some(tokio::spawn(logger_builder.build_tui(ctx)));
					}

					None
				}


				pub(super) async fn database(
					ctx: app::AppContextWriter,
					args: #params_ty,
					ty: impl Into<DatabaseType>,
				) -> database::Result<database::Client> {
					match ty.into() {
						| DatabaseType::Relational => {
							let cfg = config::load_or_prompt::<config::DatabaseConfig>(
								constants::CONFIG_DATABASE,
							)?;

							database::connect(
								(cfg.ip, cfg.port),
								(cfg.username, cfg.password),
								cfg.name,
							).await
						}
						| DatabaseType::FileSystem => {
							todo!("database: filesystem")
						}
					}
				}
			}
		})
	}

	fn handle_path<'a>(
		&self,
		meta: &'a Meta,
		path: &'a Path,
		arg_lit: String,
	) -> Result<'a, TokenStream2> {
		let ident = path
			.get_ident()
			.expect("Devrait être un identifiant valide");
		if !Self::SUPPORT_ATTRIBUTES.contains(&ident.to_string().as_str()) {
			return Err(Error::UnknownAttribute(ident, meta.span()));
		}

		let args_pat = {
			let mut list = Punctuated::new();
			if let Some(i) = self.get_first_arg_pat() {
				list.push(i);
			}
			if let Some(i) = self.get_last_arg_pat() {
				list.push(i);
			}
			PatTuple {
				attrs: Default::default(),
				paren_token: syn::token::Paren(meta.span()),
				elems: list,
			}
		};

		let maybe_ident = Ident::new(&format!("maybe_{ident}"), ident.span());
		Ok(quote! {
			let (ctx, mut crx) = tokio::sync::mpsc::channel(32);
			#[allow(unused_variables)]
			let #maybe_ident = setup::#ident(ctx.clone(), #args_pat, #arg_lit).await;
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

	fn get_first_arg_ty(&self) -> Option<Type> {
		let inputs = &self.input.sig.inputs;

		if inputs.is_empty() {
			return None;
		}

		let first_argument = inputs.first().and_then(|arg| match arg {
			| FnArg::Typed(paty) => Some(paty.ty.clone()),
			| FnArg::Receiver(_) => None,
		});

		first_argument.map(|boxed_type| match &*boxed_type {
			| Type::Path(_) => Type::Reference(TypeReference {
				and_token: syn::token::And(boxed_type.span()),
				lifetime: Default::default(),
				mutability: Default::default(),
				elem: boxed_type,
			}),
			| _ => unreachable!("#[phisyrc::setup]: get_first_arg_ty"),
		})
	}

	fn get_last_arg_ty(&self) -> Option<Type> {
		let inputs = &self.input.sig.inputs;

		if inputs.len() != Self::TOTAL_ARGUMENTS_EXPECTED {
			return None;
		}

		let last_argument = inputs.last().and_then(|arg| match arg {
			| FnArg::Typed(paty) => Some(paty.ty.clone()),
			| FnArg::Receiver(_) => None,
		});

		last_argument.map(|boxed_type| match &*boxed_type {
			| Type::Path(_) => Type::Reference(TypeReference {
				and_token: syn::token::And(boxed_type.span()),
				lifetime: Default::default(),
				mutability: Default::default(),
				elem: boxed_type,
			}),
			| _ => unreachable!("#[phisyrc::setup]: get_last_arg_ty"),
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
			.filter(|arg| matches!(arg, syn::FnArg::Typed(_)))
			.and_then(|arg| match arg {
				| syn::FnArg::Typed(typed) => Some((&typed.pat, &typed.ty)),
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
			| Self::UnknownAttribute(attr_ident, _) => {
				format!("l'attribut `{attr_ident}` n'est pas reconnu.")
			}
		};
		write!(f, "#[phisyrc::setup]: {reason}")
	}
}
