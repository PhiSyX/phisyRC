/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import type { Option } from "std/option";

/// Focus le bouton d'ajout de salon lors du click sur l'champ de recherche
/// ou lors de l'appui des touches du clavier.
function focus_button_channel(
	evt: MouseEvent | KeyboardEvent,
	maybe_channel_list_btn: Option<HTMLElement>,
) {
	if (evt.ctrlKey || evt.shiftKey || evt.altKey || evt.metaKey) {
		return;
	}

	// @ts-expect-error : n'existe pas en MouseEvent
	if (evt.code === "Tab") {
		return;
	}

	evt.preventDefault();

	if (maybe_channel_list_btn.is_some()) {
		let $channel_list_btn = maybe_channel_list_btn.unwrap();
		$channel_list_btn.click();
		$channel_list_btn.focus();
	}
}

/// Définis les salons sélectionnés dans l'état local du composant.
function set_selected_channel(
	evt: MouseEvent,
	local_state: Vec<usize>,
	chan_idx: usize,
): Vec<number> {
	if (local_state.includes(chan_idx)) {
		return local_state.filter((x) => x !== chan_idx);
	}

	if (evt.ctrlKey) {
		local_state = [...local_state, chan_idx];
	} else {
		local_state = [chan_idx];
	}

	return local_state;
}

// ------ //
// Export //
// ------ //

export { focus_button_channel, set_selected_channel };
