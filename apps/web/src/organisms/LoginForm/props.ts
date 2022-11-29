/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import { Option } from "@phisyrc/std";

type Channel = {
	id: string | number;
	name: string;
	topic: string;

	is_bookmarked: boolean;
	is_checked: boolean;
	image_url: Option<`http${"s" | ""}://${string}`>;
};

type Props = {
	nickname: string;
	server_password: string;
	channels: Channel[];
};

export type { Props };
