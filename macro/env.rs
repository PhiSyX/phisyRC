/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;

use proc_macro::TokenStream;
use syn::{
	Field,
	__private::{
		quote::{quote, quote_spanned},
		Span, TokenStream2,
	},
	punctuated::Punctuated,
	spanned::Spanned,
	token::Comma,
	Attribute, Lit, NestedMeta,
};

use crate::utils::{attribute, field};

// ---- //
// Type //
// ---- //

type Input = syn::ItemStruct;

type Result<'a, T> = core::result::Result<T, Error<'a>>;

// --------- //
// Structure //
// --------- //

pub(super) struct Analyzer {
	pub(super) input: Input,
}

// ----------- //
// Énumération //
// ----------- //

pub(super) enum Error<'a> {
	/// La structure utilisateur n'est pas une structure de champs nommés.
	IsNotNamedStruct(Span),
	/// La structure utilisateur n'est pas une structure de champs nommés.
	IsNotContainsLiteral(&'a Field, Span),
	/// La structure est mal formée.
	Parse(&'a Field, Span),
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl Analyzer {
	/// Initialise la structure [Self].
	pub(super) fn new(input: Input) -> Self {
		Self { input }
	}

	/// Construit la structure utilisateur.
	pub(super) fn build(&self) -> Result<TokenStream> {
		if !field::is_structure_of_fields_named(&self.input.fields) {
			return Err(Error::IsNotNamedStruct(self.input.span()));
		}

		let fields: Vec<_> = self
			.input
			.fields
			.iter()
			.map(|field| self.parse_field(field))
			.collect::<Result<_>>()?;

		let struct_ident = &self.input.ident;
		let output = quote! {
			// NOTE(phisyx): `env` DOIT être en dépendance du module en question.
			impl env::Interface for #struct_ident {
				fn setup(filename: impl AsRef<::std::path::Path>) -> Result<Self, env::Error> {
					env::Parser::file(filename)
						.expect("Impossible d'analyser le fichier d'environnement");
					let config = Self { #(#fields,)* };
					Ok(config)
				}
			}
		};

		Ok(output.into())
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
// Implémentation // -> Privée
// -------------- //

impl Analyzer {
	/// Nom de l'attribut `var`.
	//
	// -> #[var()]
	const ATTR_NAME: &'static str = "var";

	/// Cherche l'attribut [Self::ATTR_NAME] parmi la liste des attributs d'un
	/// champ.
	fn find_attr<'a>(&'a self, field: &'a Field) -> Option<&Attribute> {
		field
			.attrs
			.iter()
			.find(|attr| attr.path.is_ident(Self::ATTR_NAME))
	}

	/// Initialise une valeur pour un champ.
	fn initialize_value_for_field<'a>(
		&self,
		field: &'a Field,
		token_stream: TokenStream2,
		maybe_lit: Option<&'a Lit>,
	) -> Result<TokenStream2> {
		let field_ident = &field.ident;

		if let syn::Type::Path(type_path) = &field.ty {
			if let Some(first_ident) =
				type_path.path.segments.first().map(|fs| &fs.ident)
			{
				if first_ident == "Option" {
					let struct_ident = &self.input.ident;

					return Ok(quote! {
						#field_ident: #struct_ident::get_optional_var(#token_stream)?
					});
				}
			}
		}

		let struct_ident = &self.input.ident;
		if let Some(default) = maybe_lit {
			Ok(quote! {
				#field_ident: #struct_ident::get_default_var(#token_stream, #default)?
			})
		} else {
			Ok(quote! {
				#field_ident: #struct_ident::get_var(#token_stream)?
			})
		}
	}

	fn parse_field<'a>(&'a self, field: &'a Field) -> Result<'_, TokenStream2> {
		let maybe_attr = self.find_attr(field);

		if maybe_attr.is_none() {
			let token = field::token_upper(field);
			return self.initialize_value_for_field(field, token, None);
		}

		let nested_list = maybe_attr
			.map(|attr| {
				attr.parse_meta()
					.and_then(|meta| {
						if let syn::Meta::List(list) = meta {
							Ok(list.nested)
						} else {
							Err(syn::Error::new(
								attr.span(),
								"une liste est attendue",
							))
						}
					})
					.map_err(|_| {
						Error::IsNotContainsLiteral(field, attr.span())
					})
			})
			.ok_or_else(|| Error::Parse(field, field.span()))??;

		let env_key_token = Self::get_env_key(field, &nested_list);
		let default_attr = attribute::get_prop_in_list(&nested_list, "default");

		self.initialize_value_for_field(field, env_key_token, default_attr)
	}

	pub fn get_env_key(
		field: &Field,
		nested_list: &Punctuated<NestedMeta, Comma>,
	) -> TokenStream2 {
		let key_attr = attribute::get_prop_in_list(nested_list, "key");
		match key_attr {
			| Some(val) => quote! { #val },
			| None => field::token_upper(field),
		}
	}
}

impl<'a> Error<'a> {
	fn field(&self) -> &'a Field {
		match self {
			| Self::IsNotNamedStruct(_) => {
				panic!("ne devrait jamais tomber ici.");
			}
			| Self::IsNotContainsLiteral(field, _) => field,
			| Self::Parse(field, _) => field,
		}
	}

	fn span(self) -> Span {
		match self {
			| Self::IsNotNamedStruct(span)
			| Self::IsNotContainsLiteral(_, span)
			| Self::Parse(_, span) => span,
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'a> fmt::Display for Error<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"#[derive(Env)]: {}",
			match self {
				| Self::IsNotNamedStruct(_) => {
					"le trait dérivable n'est supporté que pour les structures
					de champs nommés."
						.to_owned()
				}
				| Self::IsNotContainsLiteral(..) => {
					format!(
						"l'attribut '{}' du champ '{}' DOIT contenir \
						une valeur littérale. (ex: {})",
						Analyzer::ATTR_NAME,
						field::name(self.field()),
						r#"#[var(<name> = "hello world", ...)]"#
					)
				}
				| Self::Parse(..) => {
					format!(
						"impossible d'analyser les options/paramètres de \
						l'attribut '{}' pour le champ '{}'",
						Analyzer::ATTR_NAME,
						field::name(self.field()),
					)
				}
			}
		)
	}
}
