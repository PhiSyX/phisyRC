/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

function rounded(n: isize, precision: u32): isize {
	const { pow: p, round: r } = Math;
	const accurate = p(10, precision);
	return r(n * accurate) / accurate;
}

function format(n: isize): str {
	const { abs: a, floor: f, log: l, min, pow: p } = Math;
	const ABBREVIATIONS = "kmb";
	let base = f(l(a(n)) / l(1000));
	let suffix = ABBREVIATIONS[min(2, base - 1)];
	base = ABBREVIATIONS.indexOf(suffix) + 1;
	return suffix ? rounded(n / p(1000, base), 2) + suffix : n.toFixed();
}

/**
 * Rend un nombre plus "user friendly"
 */
function to_user_friendly(n: isize): str {
	return format(n);
}

// ------ //
// Export //
// ------ //

export { to_user_friendly };

// ---- //
// Test //
// ---- //

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest;

	it("to_user_friendly", () => {
		expect(to_user_friendly(0)).toBe("0");
		expect(to_user_friendly(1)).toBe("1");
		expect(to_user_friendly(150)).toBe("150");

		expect(to_user_friendly(1500)).toBe("1.5k");
		expect(to_user_friendly(1542)).toBe("1.54k");
		expect(to_user_friendly(1549)).toBe("1.55k");

		expect(to_user_friendly(15490)).toBe("15.49k");
		expect(to_user_friendly(154900)).toBe("154.9k");
		expect(to_user_friendly(1549000)).toBe("1.55m");
		expect(to_user_friendly(15490000)).toBe("15.49m");
	});
}
