/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import type { App as Vue } from "vue";
import type { RouteRecordRaw } from "vue-router";

import type { Selector, SetupInterface } from "~/app";
import type { Route } from "~/routes";

import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { createPinia } from "pinia";

import BaseApp, { System } from "~/app";
import LayoutDefaultComponent from "./layouts/LayoutDefault.vue";
import { Option } from "@phisyrc/std/option";

class App extends BaseApp<Framework> {
	#fw: Framework;

	constructor() {
		let fw = new Framework();
		super(fw);
		this.#fw = fw;
	}

	mount(): Option<Selector> {
		return super.mount().map((s) => {
			this.#fw.lib.mount(s);
			return s;
		});
	}
}

class Framework implements SetupInterface {
	lib!: Vue;
	routes: RouteRecordRaw[] = [];

	// ----------- //
	// Constructor //
	// ----------- //

	constructor() {
		// TODO(phisyx): on pourrait imaginer avoir plusieurs layouts;
		this.lib = createApp(LayoutDefaultComponent);
	}

	// ------- //
	// Méthode // -> API Publique
	// ------- //

	public setup(system: System) {
		switch (system) {
			case System.Router: {
				let router_options = {
					history: createWebHistory(),
					routes: this.routes,
				};
				this.lib.use(createRouter(router_options));
				break;
			}

			case System.Store: {
				this.lib.use(createPinia());
				break;
			}
		}
	}

	public set_routes(routes: Route) {
		let vue_routes: RouteRecordRaw[] = [
			{
				path: "/design-system/:dyncomponent?",
				component: () => import("~vue/pages/design-system.vue"),
				strict: true,
				props: true,
			},
			{
				path: "/",
				component: () => import("~vue/pages/home.vue"),
				strict: true,
				name: routes["/"]["name"],
				children: [
					{
						path: "",
						component: () => import("~vue/pages/login.vue"),
						strict: true,
						name: routes["/login"]["name"],
						meta: {
							sidebar: false,
						},
					},

					{
						path: "chat",
						component: () => import("~vue/pages/chat.vue"),
						strict: true,
						name: routes["/chat"]["name"],
						meta: {
							sidebar: true,
						},
					},
				],
			},
		];
		this.routes.push(...vue_routes);
	}
}

// ------ //
// Export //
// ------ //

export default App;
