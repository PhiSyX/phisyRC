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

	static Some = <T>(value: T) => {
		if (value == null) {
			return None;
		}
		return new this(Variant.Some, value);
	};

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

	expect(msg: string): safety<T> {
		if (this.is_some()) {
			return this.value!;
		}
		throw new Error(msg);
	}

	/// La valeur de l'option n'est pas `nil`.
	is_some(): this is Option<safety<T>> {
		return this.value != null;
	}

	/// La valeur de l'option est `nil`.
	is_none(): this is Option<never> {
		return this.value == null;
	}

	/// Modifie la valeur contenue dans [`Some`].
	map<U>(f: (_: safety<T>) => U): Option<U> {
		if (this.is_some()) {
			return Some(f(this.value!));
		}
		return None;
	}

	/// Retourne la valeur contenue dans [Some]
	unwrap() {
		const ERROR_MESSAGE: string =
			"La fonction `.unwrap()` est appelée sur une valeur `None`.";
		return this.expect(ERROR_MESSAGE);
	}

	/// Retourne la valeur contenu dans [Some] ou une valeur par défaut.
	unwrap_or<U>(def: safety<U>): safety<U> {
		if (this.is_some()) {
			return this.value! as safety<U>;
		}
		return def;
	}
}

const { Some, None } = Option;

// ------ //
// Export //
// ------ //

export { Option, None, Some };

// ---- //
// Test //
// ---- //

if (import.meta.vitest) {
	const { it, expect } = import.meta.vitest;

	it("Option#Some", () => {
		expect(Some("")).toEqual(Some(""));
		expect(Some(null)).toEqual(None);
		expect(Some(undefined)).toEqual(None);
	});

	it("Option#None", () => {
		expect(None).toEqual(None);
	});

	it("Option#from", () => {
		expect(Option.from(null)).toEqual(None);

		expect(Option.from("hello")).toEqual(Some("hello"));
	})

	it("Option#{is_some, is_none}", () => {
		expect(Some("").is_some()).toBeTruthy();
		expect(Some("").is_none()).toBeFalsy();

		expect(None.is_some()).toBeFalsy();
		expect(None.is_none()).toBeTruthy();
	});

	it("Option#map", () => {
		expect(Some("Hello").map((hello) => `${hello} World`)).toEqual(Some("Hello World"));

		expect(None.map((hello) => `${hello} World`)).toEqual(None);
	});

	it("Option#unwrap", () => {
		expect(Some("hello").unwrap()).toBe("hello");
		expect(() => None.unwrap()).toThrowError(
			"La fonction `.unwrap()` est appelée sur une valeur `None`.",
		);
	});

	it("Option#unwrap_or", () => {
		expect(Some("hello").unwrap_or("world")).toBe("hello");
		expect(None.unwrap_or("world")).toBe("world");
	});
}
