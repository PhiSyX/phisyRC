/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// ---- //
// Type //
// ---- //

type Route = {
	[P in `/${string}`]: RouteRecord;
};

type RouteRecord = { name?: string };

// -------- //
// Constant //
// -------- //

const ROUTES: Route = {
	// TODO(phisyx): Page de login
	// "/login": { name: "login" },
	//
	// Page d'accueil
	"/": { name: "home" },
};

// ------ //
// Export //
// ------ //

export type { Route, RouteRecord };

export { ROUTES };
