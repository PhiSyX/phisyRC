/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import "design/style.scss";

import { ExitCode } from "../std/process";

import Vue from "../vue/app";

async function main<T>(argv?: Vec<T>): Future<ExitCode> {
	// TODO(phisyx): utiliser un logger.

	let app = new Vue();

	return app
		.mount()
		.map((_) => ExitCode.SUCCESS)
		.unwrap_or(ExitCode.FAILURE);
}

main()
	.then((code) => {
		if (code === ExitCode.FAILURE) {
			throw new Error("error: exit failure");
		}

		console.log("let's go.");
	})
	.catch(console.error);
