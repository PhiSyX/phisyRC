/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

function toggle_click(
	evt: MouseEvent,
	/*$1*/ toggle_value: boolean,
	notifier: ($1: boolean) => void,
) {
	notifier(toggle_value);

	if (toggle_value === false) {
		let target = evt.currentTarget as HTMLButtonElement;
		setTimeout(() => target.blur(), 0);
	}
}

export { toggle_click };
