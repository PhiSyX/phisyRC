/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

declare type nil = null | undefined;
declare type unsafe<T> = T | nil;
declare type safety<T> = T extends Array<infer U>
	? safety<U>[]
	: NonNullable<T>;


/**
 * Rend certaines propriétés optionnelles.
 *
 * @example ```ts
 *   interface Params {
 *     foo: string;
 *     bar: boolean;
 *   }
 *
 *   const params: Optional<Params, "bar"> = { foo: "bar" };
 *   console.log(params.foo);
 * ```
 *
 * Contrairement au type `Partial`, ce type ne rend pas TOUTES les propriétés
 * d'un objet optionnelles.
 */
declare type Optional<T, K extends keyof T> = Partial<T> & Omit<T, K>;
