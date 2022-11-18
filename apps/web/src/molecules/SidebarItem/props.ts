/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import type { Room } from "~/types";

type Props = {
	type: Room["type"];
};

interface ChannelProps extends Room {
	type: "channel",
	name: `#${Room["name"]}`;
};

interface PrivateProps extends Optional<
	Room,
	| "is_active"
	| "is_highlighted"
	| "total_unread_event"
	| "total_unread_message"
> {
	type: "private",
	last_message: NonNullable<Room["last_message"]>;
}

type ServerProps = {
	name: string;
	is_folded: boolean;
};

export type { Props, ChannelProps, PrivateProps, ServerProps };
