/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/**
 * Remplace tous les premiers caractères de mots d'une phrase par une
 * majuscule, et le reste en minuscule.
 */
function capitalize(s: str): string {
	if (s.length === 0) {
		return s;
	}

	const algo = (str: string) => {
		// SAFETY: la condition ci-haut nous garantie que la chaîne de
		// caractères comporte au moins 1 caractère, qui permet d'accéder à
		// l'index 0 de la chaîne.
		//
		// NOTE(phisyx): le résultat d'une [String.prototype.slice(1)] lorsque
		// la chaîne est vide, renvoie une chaîne vide.
		return str[0].toUpperCase() + str.slice(1).toLowerCase();
	};

	const SEPARATOR = /([\s-_]+)/;

	if (!SEPARATOR.test(s)) {
		return algo(s);
	}

	return s.split(SEPARATOR).filter(Boolean).map(algo).join("");
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

	it("capitalize", () => {
		expect(capitalize("")).toBe("");
		expect(capitalize("h")).toBe("H");
		expect(capitalize("hello ")).toBe("Hello ");
		expect(capitalize("hello world")).toBe("Hello World");
		expect(capitalize("HELLO WORLD")).toBe("Hello World");
	});
}
