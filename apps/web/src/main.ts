/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import "design/style.scss";

import { ExitCode } from "../std/process";

import { UI } from "./app";
import Vue from "../vue/app";

// TODO(phisyx): utiliser un logger.
async function main<T>(...argv: Vec<T>): Future<ExitCode> {
	if (argv.length !== 1) {
		return ExitCode.FAILURE;
	}

	// SAFETY: on peut se permettre de déstructurer, car la condition ci-haut
	// permet d'être certain qu'il y a les arguments nécessaires.
	let [ui] = argv;

	switch (ui) {
		case UI.Vue: {
			let app = new Vue();

			return app
				.mount()
				.map((_) => ExitCode.SUCCESS)
				.unwrap_or(ExitCode.FAILURE);
		}
	}

	return ExitCode.FAILURE;
}

main(UI.Vue).then((code) => {
	if (code === ExitCode.FAILURE) {
		throw new Error("error: exit failure");
	}

	console.log("let's go.");
})
	.catch(console.error);
