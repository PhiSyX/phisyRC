/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// ---- //
// Type //
// ---- //

type Options = {
	/// Remplace tout le reste d'une chaîne de caractères en minuscule.
	to_lower?: bool;
	/// Inclus les séparateurs dans le résultat?
	includes_separators?: bool;
};

// -------- //
// Constant //
// -------- //

const INCLUDE_SEPARATOR: RegExp = /([\s-_]+)/;
const EXCLUDE_SEPARATOR: RegExp = /[\s-_]+/;

const Default: Options = {
	to_lower: true,
	includes_separators: true,
};

/**
 * Remplace tous les premiers caractères des mots d'une chaîne de caractères |s|
 * par une majuscule.
 */
function capitalize(s: str, user_options: Options = Default): str {
	let options: Options = { ...Default, ...user_options };

	const algo = (s: str) => {
		if (s.length === 0) {
			return s;
		}

		// SAFETY: la condition ci-haut nous garantie que la chaîne de
		// caractères comporte au moins 1 caractère, qui nous permet d'accéder à
		// l'index 0 de la chaîne en toute sécurité.
		let first_ch = s[0].toUpperCase();

		// NOTE(phisyx): le résultat d'une [String.prototype.slice(1)] lorsque
		// la chaîne est vide, renvoie une chaîne vide.
		let rest_of_str = s.slice(1);

		if (options.to_lower) {
			return first_ch + rest_of_str.toLowerCase();
		} else {
			return first_ch + rest_of_str;
		}
	};

	if (!INCLUDE_SEPARATOR.test(s)) {
		return algo(s);
	}

	if (options.includes_separators === true) {
		return s.split(INCLUDE_SEPARATOR).map(algo).join("");
	}

	return s.split(EXCLUDE_SEPARATOR).map(algo).join("");
}

// ------ //
// Export //
// ------ //

export { capitalize };

// ---- //
// Test //
// ---- //

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest;

	it("capitalize: vide", () => {
		expect(capitalize("")).toEqual("");
	});

	it("capitalize: cas basiques", () => {
		expect(capitalize(" ")).toEqual(" ");
		expect(capitalize("h")).toEqual("H");
		expect(capitalize("hello world")).toEqual("Hello World");
		expect(capitalize("HELLO WORLD")).toEqual("Hello World");
	});

	it("capitalize: espace en trop", () => {
		expect(capitalize("hello ")).toEqual("Hello ");

		expect(capitalize("hello    ")).toEqual("Hello    ");
		expect(capitalize("    hello    ")).toEqual("    Hello    ");
	});

	it("capitalize: garde le reste de la chaîne de caractères intacte", () => {
		expect(capitalize("HeLLo WorLd", { to_lower: false })).toEqual(
			"HeLLo WorLd",
		);
	});

	it("capitalize: exclusion des séparateurs dans le résultat", () => {
		expect(capitalize("hello ", { includes_separators: false })).toEqual(
			"Hello",
		);

		expect(
			capitalize("hello world", {
				includes_separators: false,
			}),
		).toEqual("HelloWorld");

		expect(
			capitalize("    hello    ", {
				includes_separators: false,
			}),
		).toEqual("Hello");
	});
}
