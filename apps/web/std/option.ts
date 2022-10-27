/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// ----------- //
// Énumération //
// ----------- //

const enum Variant {
	Some = "Some",
	None = "None",
}

// -------------- //
// Implémentation //
// -------------- //

class Option<T> {
	// ------ //
	// Static //
	// ------ //

	static None = new this<never>(Variant.None);

	static Some = <T>(value: T) => new this(Variant.Some, value);

	static from = <T>(value: T) => {
		if (value == null) {
			return this.None;
		}
		return this.Some(value);
	};

	// ----------- //
	// Constructor //
	// ----------- //
	constructor(public type: Variant, private value?: unsafe<T>) { }

	expect(msg: str): safety<T> {
		if (this.is_some()) {
			return this.value!;
		}
		throw new Error(msg);
	}

	is_some(): this is Option<safety<T>> {
		return this.value != null;
	}

	is_none(): this is Option<never> {
		return this.value == null;
	}

	map<U>(f: (_: T) => U): Option<U> {
		if (this.is_some()) {
			return Some(f(this.value!));
		}
		return None;
	}

	unwrap_or(def: T): T {
		if (this.is_some()) {
			return this.value!;
		}
		return def;
	}
}

const { Some, None } = Option;

// ------ //
// Export //
// ------ //

export { Option, None, Some };
