/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Primitive

declare type bool = boolean;
declare type str = string;

// Unsigned integer types
declare type usize = number;
declare type u8 = number;
declare type u16 = number;
declare type u32 = number;
declare type u64 = number;

// Signed integer types
declare type isize = number;
declare type i8 = number;
declare type i16 = number;
declare type i32 = number;
declare type i64 = number;

// Array
declare type FixedArray<T, /*const*/ N extends usize> = T[];
declare type Tuple<T extends unknown[]> = [...T];
declare type Vec<T> = T[];
declare type HashMap<K, V> = Map<K, V>;

// Promise
declare type Future<T> = Promise<T>;
