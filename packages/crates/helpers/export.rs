/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{
	algorithms::SHA2,
	lang::WildcardMatching,
	string::{Escape, Slugify},
};

impl<T: AsRef<str>> Escape for T {}
impl<T: AsRef<str>> WildcardMatching for T {}
impl<T: AsRef<str>> SHA2 for T {}
impl<T: AsRef<str>> Slugify for T {}
