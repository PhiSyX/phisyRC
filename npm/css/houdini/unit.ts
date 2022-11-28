/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// CSS.px houdini
function to_px(n: usize): CSSUnitValue {
	if (typeof CSS?.px !== "undefined") {
		return CSS.px(n);
	}
	return `${n}px`;
}

export { to_px };
