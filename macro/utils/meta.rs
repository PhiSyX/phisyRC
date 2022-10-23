/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use syn::{
	punctuated::Punctuated, token, Attribute, Meta, MetaNameValue, NestedMeta,
};

pub fn get_list(
	attr: &Attribute,
) -> Option<Punctuated<NestedMeta, token::Comma>> {
	attr.parse_meta().ok().and_then(|meta| {
		if let Meta::List(list) = meta {
			return Some(list.nested);
		}
		None
	})
}

pub fn get_name_value(attr: &Attribute) -> Option<MetaNameValue> {
	attr.parse_meta().ok().and_then(|meta| {
		if let Meta::NameValue(meta_nv) = meta {
			return Some(meta_nv);
		}
		None
	})
}

pub fn has<const N: usize>(
	attr: &Attribute,
	expected_meta: [&'static str; N],
) -> bool {
	attr.parse_meta()
		.map(|meta| match meta {
			| Meta::Path(_) if expected_meta.contains(&"path") => true,
			| Meta::List(_) if expected_meta.contains(&"list") => true,
			| Meta::NameValue(_) if expected_meta.contains(&"name=value") => {
				true
			}
			| _ => false,
		})
		.unwrap_or_default()
}
