/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

export type User = {
	nick: string;
};

export type Message = {
	type: "action" | "privmsg" | "event";
	message: string;
	from: User;
};

export type Room = {
	type: "channel" | "private" | "custom-window";
	name: string;
	is_active: boolean;
	is_highlighted: boolean;
	total_unread_event: number;
	total_unread_message: number;
	last_message?: Message;
};

export type Server = {
	name: string;
	is_focused: boolean;
	is_folded: boolean;
	rooms: Room[];
};
