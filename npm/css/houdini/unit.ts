/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// CSS.px houdini
function to_px(n: isize): CSSUnitValue {
	// NOTE(phisyx): `CSS` est disponible dans l'objet Window dans certains
	// navigateurs.
	if (typeof globalThis.CSS?.px !== "undefined") {
		return globalThis.CSS.px(n);
	}
	return `${n}px`;
}

// ------ //
// Export //
// ------ //

export { to_px };

// ---- //
// Test //
// ---- //

if (import.meta.vitest) {
	const { expect, it, beforeEach } = import.meta.vitest;

	beforeEach(() => {
		globalThis.CSS = {
			px(n: isize): CSSUnitValue {
				return { unit: "px", value: n }
			}
		};
	});

	it("to_px: CSS", () => {
		expect(to_px(42)).toEqual({ unit: "px", value: 42 });
	});

	it("to_px: string", () => {
		// @ts-ignore : osef
		globalThis.CSS = undefined;

		expect(to_px(42)).toEqual("42px");
	});
}
