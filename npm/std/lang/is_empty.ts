/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

type OneOfProperty = {
	count?: usize;
	length?: usize
	size?: usize
};

/// Vérifie si la valeur passée par argument |value| est vide.
export function is_empty<T extends OneOfProperty>(value: T): value is T {
	return value.length === 0 || value.count === 0 || value.size === 0;
}
