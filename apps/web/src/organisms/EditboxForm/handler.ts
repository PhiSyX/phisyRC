/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import { is_empty } from "@phisyrc/std";

type InputOptions = {
	data: str;
	update: (_: str) => void;
};

type HistoryOptions = {
	data: str[];
	current: isize;
	update: (_: isize) => void;
};

function handle_keydown(
	evt: KeyboardEvent,
	input: InputOptions,
	history: HistoryOptions,
) {
	handle_keydown_history(evt, input, history);
}

function handle_keydown_history(
	evt: KeyboardEvent,
	input: InputOptions,
	history: HistoryOptions,
) {
	if (!(is_empty(input.data) || history.data.includes(input.data))) {
		return;
	}

	let history_size = history.data.length;
	let history_current = history.current;

	switch (evt.code.toLowerCase()) {
		case "arrowup": {
			history_current -= 1;
			if (history_current <= 0) {
				history_current = 0;
			}
			history.update(history_current);
			break;
		}

		case "arrowdown": {
			history_current += 1;
			if (history_current >= history_size) {
				history_current = history_size;
			}
			history.update(history_current);
			break;
		}

		default:
			return;
	}

	let new_input = history.data[history_current] || "";
	input.update(new_input);
}

export { handle_keydown };
