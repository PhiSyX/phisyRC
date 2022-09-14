/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

use proc_macro::TokenStream;
use syn::{
	__private::{
		quote::{quote, quote_spanned},
		Span, ToTokens, TokenStream2,
	},
	spanned::Spanned,
};

// --------- //
// Structure //
// --------- //

pub(super) struct EnvAnalyzer {
	pub(super) struct_input: syn::ItemStruct,
}

// ----------- //
// Énumération //
// ----------- //

pub(super) enum EnvAnalyzerError<'f> {
	IsNotNamedStruct(Span),
	IsNotContainsLiteral(Span, &'f syn::Field),
	ParseError(Span, &'f syn::Field),
}

// -------------- //
// Implémentation //
// -------------- //

impl EnvAnalyzer {
	/// Nom de l'attribut `var`.
	//
	// -> #[var()]
	const ATTR_NAME: &'static str = "var";

	/// Crée un nouveau [EnvAnalyzer].
	pub(super) fn new(struct_input: syn::ItemStruct) -> Self {
		Self { struct_input }
	}

	/// Cherche l'attribut [Self::ATTR_NAME] parmi la liste des attributs d'un
	/// champ.
	fn find_attr<'a>(
		&'a self,
		field: &'a syn::Field,
	) -> Option<&syn::Attribute> {
		field
			.attrs
			.iter()
			.find(|attr| attr.path.is_ident(Self::ATTR_NAME))
	}

	/// Récupère une propriété d'une liste d'attributs.
	//
	// -> #[var( prop = "value", prop2 = "value2" )]
	fn get_prop_in_attrlist<'f, 'n>(
		&'f self,
		list: &'n syn::punctuated::Punctuated<
			syn::NestedMeta,
			syn::token::Comma,
		>,
		prop: &'n str,
	) -> Option<&'n syn::Lit> {
		list.iter().find_map(|nested_meta| {
			if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) =
				nested_meta
			{
				name_value.path.is_ident(prop).then_some(&name_value.lit)
			} else {
				None
			}
		})
	}

	fn is_named_fields(&self) -> bool {
		matches!(self.struct_input.fields, syn::Fields::Named(_))
	}

	/// Initialise une valeur pour un champ.
	fn initialize_value_for_field<'a>(
		&self,
		field: &'a syn::Field,
		token_stream: TokenStream2,
		maybe_lit: Option<&'a syn::Lit>,
	) -> Result<TokenStream2, EnvAnalyzerError> {
		let field_ident = &field.ident;

		if let syn::Type::Path(type_path) = &field.ty {
			if let Some(first_ident) =
				type_path.path.segments.first().map(|fs| &fs.ident)
			{
				if first_ident == "Option" {
					let struct_ident = &self.struct_input.ident;

					return Ok(quote! {
						#field_ident: #struct_ident::get_optional_var(#token_stream)?
					});
				}
			}
		}

		let struct_ident = &self.struct_input.ident;
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

	pub(super) fn parse(&self) -> Result<TokenStream, EnvAnalyzerError<'_>> {
		if !self.is_named_fields() {
			return Err(EnvAnalyzerError::IsNotNamedStruct(
				self.struct_input.span(),
			));
		}

		let maybe_fields = self
			.struct_input
			.fields
			.iter()
			.map(|field| self.parse_field(field));

		let mut fields = Vec::with_capacity(maybe_fields.len());
		for field in maybe_fields {
			fields.push(field?);
		}

		let struct_ident = &self.struct_input.ident;
		let output = quote! {
			impl EnvInterface for #struct_ident {
				fn setup(filename: impl AsRef<::std::path::Path>) -> Result<Self, EnvError> {
					EnvParser::file(filename)
						.expect("Impossible d'analyser le fichier d'environnement");
					let config = Self { #(#fields,)* };
					Ok(config)
				}
			}
		};

		Ok(output.into())
	}

	fn parse_field<'a>(
		&'a self,
		field: &'a syn::Field,
	) -> Result<TokenStream2, EnvAnalyzerError<'_>> {
		let maybe_attr = self.find_attr(field);

		if maybe_attr.is_none() {
			let token = token_upper(field);
			return self.initialize_value_for_field(field, token, None);
		}

		let nested_list = maybe_attr
			.map(|attr| {
				attr.parse_meta()
					.and_then(|meta| {
						if let syn::Meta::List(list) = meta {
							Ok(list.nested)
						} else {
							Err(syn::Error::new(attr.span(), "expected a list"))
						}
					})
					.map_err(|_| {
						EnvAnalyzerError::IsNotContainsLiteral(
							attr.span(),
							field,
						)
					})
			})
			.ok_or_else(|| {
				EnvAnalyzerError::ParseError(field.span(), field)
			})??;

		let default_attr = self.get_prop_in_attrlist(&nested_list, "default");
		let key_attr = self.get_prop_in_attrlist(&nested_list, "key");

		let field_token = match key_attr {
			| Some(val) => quote! { #val },
			| None => token_upper(field),
		};

		self.initialize_value_for_field(field, field_token, default_attr)
	}
}

impl<'a> EnvAnalyzerError<'a> {
	pub(super) fn field(&self) -> &'a syn::Field {
		match self {
			| Self::IsNotNamedStruct(_) => {
				panic!("doesn't have a field")
			}
			| Self::IsNotContainsLiteral(_, field) => field,
			| Self::ParseError(_, field) => field,
		}
	}

	pub(super) fn span(self) -> Span {
		match self {
			| Self::IsNotNamedStruct(span)
			| Self::IsNotContainsLiteral(span, _)
			| Self::ParseError(span, _) => span,
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

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'a> fmt::Display for EnvAnalyzerError<'a> {
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
						EnvAnalyzer::ATTR_NAME,
						field_name(self.field()),
						r#"#[var(<name> = "hello world", ...)]"#
					)
				}

				| Self::ParseError(..) => {
					format!(
						"impossible d'analyser les options/paramètres de \
						l'attribut '{}' pour le champ '{}'",
						EnvAnalyzer::ATTR_NAME,
						field_name(self.field()),
					)
				}
			}
		)
	}
}

fn field_name(field: &syn::Field) -> String {
	token_to_string(&field.ident)
}

fn token_to_string<T: ToTokens>(tokens: &T) -> String {
	quote!(#tokens).to_string()
}

fn token_upper(field: &syn::Field) -> TokenStream2 {
	let field = field.ident.as_ref().unwrap().to_string().to_uppercase();
	quote! { #field }
}
