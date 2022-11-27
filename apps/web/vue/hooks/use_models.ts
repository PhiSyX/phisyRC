/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import type { WritableComputedRef } from "vue";

import { computed } from "vue";

type Props = {
	modelValue?: unknown;
	[p: PropertyKey]: unknown;
};

type EmitFn<K extends Extract<keyof Props, string>> = (
	event: `update:${K}`, ...args: any[]
) => void

function use_model<
	P extends Props,
	// @ts-expect-error : type par défaut dans le cas où aucune prop n'est
	// passée par argument ?
	K extends Extract<keyof P, string> = "modelValue",
>(
	props: P,
	prop?: K,
): (emit_fn: EmitFn<K>) => WritableComputedRef<P[K]> {
	return (emit_fn: EmitFn<K>): WritableComputedRef<P[K]> => {
		return computed({
			get(): P[K] {
				if (prop && prop in props) {
					return props[prop];
				}
				return props["modelValue"] as P[K];
			},
			set($1) {
				let uprop = (prop! || "modelValue") as K;
				emit_fn(`update:${uprop}`, $1);
			}
		});
	}
}

export { use_model };
