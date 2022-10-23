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

use crate::utils::{attribute, field, literal, meta};

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
	/// L'attribut #[prompt] est requis.
	PromptAttributeIsRequired(&'a Field, Span),
	/// Le format de l'attribut est invalide.
	FormatAttributInvalid(&'a Field, Span),
	/// Un titre est obligatoire.
	TitleIsRequired(&'a Field, Span),
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
			// NOTE(phisyx): `terminal::io` DOIT être importé dans le module
			// en question.
			impl terminal::io::Prompt for #struct_ident {
				fn prompt() -> Self {
					let build = Self { #(#fields,)* };

					println!("Configuration terminée : {:#?}", &build);
					println!();

					if terminal::io::confirm("Êtes vous satisfait de cette configuration?") {
						build
					} else {
						println!("Recommençons...");
						println!();

						Self::prompt()
					}
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
	/// Nom de l'attribut `prompt`.
	//
	// -> #[prompt()]
	const ATTR_NAME: &'static str = "prompt";

	/// Cherche l'attribut "doc" parmi la liste des attributs d'un champ.
	fn find_doc_attr<'a>(&'a self, field: &'a Field) -> Option<&Attribute> {
		attribute::find_field(field, "doc")
	}

	/// Cherche l'attribut [Self::ATTR_NAME] parmi la liste des attributs d'un
	/// champ.
	fn find_prompt_attr<'a>(&'a self, field: &'a Field) -> Result<&Attribute> {
		attribute::find_field(field, Self::ATTR_NAME).ok_or_else(|| {
			Error::PromptAttributeIsRequired(field, field.span())
		})
	}

	/// Initialise une valeur pour un champ.
	fn initialize_value_for_field<'a>(
		&self,
		field: &'a Field,
		title: &str,
		default_prop: Option<&Lit>,
	) -> Result<TokenStream2> {
		let field_ident = field.ident.as_ref();

		if let syn::Type::Path(type_path) = &field.ty {
			if let Some(first_ident) =
				type_path.path.segments.first().map(|fs| &fs.ident)
			{
				if first_ident == "Option" {
					if let Some(def) = default_prop {
						return Ok(quote! {
							#field_ident: Some(terminal::io::prompt_default(#title, #def))
						});
					}
					return Ok(quote! {
						#field_ident: terminal::io::prompt_optional(#title)
					});
				}
			}
		}

		if let Some(def) = default_prop {
			return Ok(quote! {
				#field_ident: terminal::io::prompt_default(#title, #def)
			});
		}

		Ok(quote! {
			#field_ident: terminal::io::prompt_required(#title)
		})
	}

	fn parse_field<'a>(&'a self, field: &'a Field) -> Result<TokenStream2> {
		let prompt_attr = self.find_prompt_attr(field)?;

		if !meta::has(prompt_attr, ["list", "path"]) {
			return Err(Error::FormatAttributInvalid(
				field,
				prompt_attr.span(),
			));
		}

		let maybe_meta_list = meta::get_list(prompt_attr);
		if maybe_meta_list.is_none() {
			let title = self
				.get_title_from_doc_attr(field)
				.ok_or_else(|| Error::TitleIsRequired(field, field.span()))?;
			return self.initialize_value_for_field(
				field,
				title.trim_start(),
				None,
			);
		}

		let meta_list = unsafe { maybe_meta_list.unwrap_unchecked() };

		let default_prop = attribute::get_prop_in_list(&meta_list, "default");

		let title = self.required_title(&meta_list, field)?;
		self.initialize_value_for_field(field, title.trim_start(), default_prop)
	}

	/// Récupère le titre à partir de l'attribut de la clé title de l'attribut
	/// prompt ou depuis la documentation s'il en existe une.
	fn required_title<'a>(
		&'a self,
		meta_list: &Punctuated<NestedMeta, Comma>,
		field: &'a Field,
	) -> Result<String> {
		attribute::get_prop_in_list(meta_list, "title")
			.and_then(literal::get_str)
			.or_else(|| self.get_title_from_doc_attr(field))
			.ok_or_else(|| Error::TitleIsRequired(field, meta_list.span()))
	}

	fn get_title_from_doc_attr<'a>(
		&'a self,
		field: &'a Field,
	) -> Option<String> {
		let attr = self.find_doc_attr(field)?;
		let meta_nv = meta::get_name_value(attr)?;
		literal::get_str(&meta_nv.lit)
	}
}

impl<'a> Error<'a> {
	fn span(self) -> Span {
		match self {
			| Self::IsNotNamedStruct(span)
			| Self::PromptAttributeIsRequired(_, span)
			| Self::FormatAttributInvalid(_, span)
			| Self::TitleIsRequired(_, span) => span,
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'a> fmt::Display for Error<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let attr_s = format!("#[{}]", Analyzer::ATTR_NAME);

		let err_s = match self {
			| Self::IsNotNamedStruct(_) => {
				"le trait dérivable n'est supporté que pour les structures
				de champs nommés."
					.to_owned()
			}
			| Self::PromptAttributeIsRequired(..) => {
				format!("l'attribut {attr_s} est requis pour chaque champs.")
			}
			| Self::FormatAttributInvalid(..) => {
				format!("Le format de l'attribut {attr_s} est invalide.")
			}
			| Self::TitleIsRequired(..) => {
				format!(
					"{attr_s}: une clé `title` dans l'attribut OU une documentation pour ce champ est obligatoire."
				)
			}
		};

		write!(f, "#[derive(Prompt)]: {err_s}",)
	}
}
