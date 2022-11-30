/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

type Props<HTMLAttributeClass> = {
	name: string;
	datalist?: unknown[];

	// Label Class
	lclass?: HTMLAttributeClass;
	// Root Class
	rclass?: HTMLAttributeClass;
	/// Icon Class
	iclass?: HTMLAttributeClass;
	/// DataList Class
	dclass?: HTMLAttributeClass;
	/// DataList Item Class
	diclass?: (item: any, i: usize) => HTMLAttributeClass;
	diclick?: (evt: MouseEvent, item: any, i: usize) => void;
};

export type { Props };
