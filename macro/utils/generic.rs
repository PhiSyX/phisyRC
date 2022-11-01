/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use syn::{
	GenericParam, Generics, Ident, Path, Type, TypeParam, TypeParamBound,
	WhereClause, WherePredicate, __private::Span, spanned::Spanned,
};

// ----------- //
// Énumération //
// ----------- //

pub enum Error<'a> {
	/// Le générique est manquant.
	MissingGeneric(Span),
	/// La clause where est manquante.
	MissingWhereClause(Option<&'a Ident>, Span),
}

// -------- //
// Fonction //
// -------- //

pub(crate) fn find<'a, I>(
	generic: &'a Generics,
	generic_name: &'a I,
) -> Option<&'a TypeParam>
where
	Ident: PartialEq<I>,
{
	generic.params.iter().find_map(|gen_par| {
		if let GenericParam::Type(ty_param) = gen_par &&
		   ty_param.ident.eq(generic_name) {
			return Some(ty_param);
		}
		None
	})
}

pub(crate) fn find_clause<'a, I: ?Sized>(
	where_clause: &'a WhereClause,
	generic_name: &'a I,
) -> Option<&'a Path>
where
	Ident: PartialEq<I>,
{
	where_clause.predicates.iter().find_map(|wp| {
		if let WherePredicate::Type(predicate_ty) = wp &&
		   let Type::Path(ty_path) = &predicate_ty.bounded_ty &&
		   ty_path.path.is_ident(generic_name) &&
		   let Some(TypeParamBound::Trait(trait_bound)) = predicate_ty.bounds.first()
		{
			return Some(&trait_bound.path);
		}
		None
	})
}

pub(crate) fn find_generic_and_clause<'a, I>(
	generic: &'a Generics,
	generic_name: &'a I,
) -> Result<&'a Path, Error<'a>>
where
	Ident: PartialEq<I>,
{
	let has_generic = find(generic, generic_name)
		.ok_or_else(|| Error::MissingGeneric(generic.span()))?;

	let where_clause = generic.where_clause.as_ref().ok_or_else(|| {
		Error::MissingWhereClause(None, generic.where_clause.span())
	})?;

	let has_clause = find_clause(where_clause, generic_name).ok_or_else(|| {
		Error::MissingWhereClause(Some(&has_generic.ident), where_clause.span())
	});

	has_clause
}
