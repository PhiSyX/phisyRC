/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import type { Route } from "./routes";

import { ROUTES } from "./routes";
import { Option, Some } from "../std/option";

// ---- //
// Type //
// ---- //

type Selector = string | Element;

// -------- //
// Constant //
// -------- //

const DEFAULT_SELECTOR: Selector = "#🆔";

// ----------- //
// Énumération //
// ----------- //

// NOTE(phisyx): on pourrait avoir plusieurs UI. Qui sait?
const enum UI {
	Vue = "vue",
}

enum System {
	Store,
	Router,
}

// --------- //
// Interface //
// --------- //

interface SetupInterface {
	setup(system: System): void;

	set_routes(routes: Route): void;
}

class App<T extends SetupInterface> {
	protected inner!: T;

	#mounted = false;
	#selector: Selector = DEFAULT_SELECTOR;

	// ----------- //
	// Constructor //
	// ----------- //

	constructor(user_app: T, user_selector?: Selector) {
		this.inner = user_app;

		if (user_selector) {
			this.#selector = user_selector;
		}

		this.inner.set_routes(ROUTES);

		this.inner.setup(System.Store);
		this.inner.setup(System.Router);
	}

	// --------------- //
	// Getter | Setter //
	// --------------- //

	// Sélecteur de l'élément HTML qui contiendra l'application
	get selector(): Option<Selector> {
		if (typeof this.#selector === "string") {
			return Option.from(document.querySelector(this.#selector));
		}

		return Option.from(this.#selector);
	}

	// ------- //
	// Méthode // -> API Publique
	// ------- //

	/// Monte l'application
	public mount(): Option<Selector> {
		let selector = this.selector.expect(
			"Un sélecteur d'un élément HTML valide et définie dans le DOM.",
		);

		if (!this.#mounted) {
			this.#mounted = true;
		}

		return Some(selector);
	}
}

// ------ //
// Export //
// ------ //

export type { SetupInterface, Selector };

export { UI, System };

export default App;
