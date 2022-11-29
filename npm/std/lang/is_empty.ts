/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// ---- //
// Type //
// ---- //

type OneOfProperty = {
	count?: usize;
	length?: usize
	size?: usize
};

/// Vérifie si la valeur passée par argument |value| est vide.
function is_empty<T extends OneOfProperty>(value: T): value is never {
	return value.length === 0 || value.count === 0 || value.size === 0;
}

// ------ //
// Export //
// ------ //

export { is_empty };

// ---- //
// Test //
// ---- //

if (import.meta.vitest) {
	const { expect, it } = import.meta.vitest;

	it("is_empty: .length", () => {
		expect(is_empty("")).toBeTruthy();
		expect(is_empty(" ")).toBeFalsy();
	});

	it("is_empty: .size", () => {
		expect(is_empty(new Map())).toBeTruthy();

		let map = new Map();
		map.set(1, 1);
		expect(is_empty(map)).toBeFalsy();
	});

	it("is_empty: .count", () => {
		class CustomCount1 {
			count: isize = 0;
		}

		expect(is_empty(new CustomCount1())).toBeTruthy();

		class CustomCount2 {
			count: isize = 1;
		}

		expect(is_empty(new CustomCount2())).toBeFalsy();
	});
}
