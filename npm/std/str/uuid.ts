/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Génère une chaîne de caractère avec des caractères alphanumériques
/// aléatoires de taille 36.
const uuid = (): str => get_random_str("yxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx");

/// Génère une chaîne de caractère aléatoire.
function get_random_str(s: str): str {
	const { floor: f, random: r } = Math;

	let dt = new Date().getTime();

	const replaceXY = (c: str) => {
		const random = ((dt + r() * 16) % 16) | 0;
		dt = f(dt / 16);
		return (c === "x" ? random : (random & 0x3) | 0x8).toString(16);
	};

	return s.replace(/[xy]/g, replaceXY).replace(/^\d/g, () => "x");
}

export { uuid };
