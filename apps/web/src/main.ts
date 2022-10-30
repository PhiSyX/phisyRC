/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import "design/style.scss";

import { wasm_initialize_logger } from "assets/wasm/phisyrc_wasm";

import { ExitCode } from "../std/process";
import { error, info, trace, warn, LogLevel, dbg } from "./logger"

import { UI } from "./app";
import Vue from "../vue/app";

const LOG_LEVEL: LogLevel = import.meta.env.DEV
	? LogLevel.Trace
	: LogLevel.Warn;

// TODO(phisyx): utiliser un logger.
async function main(...argv: Vec<str>): Future<ExitCode> {
	if (argv.length !== 2) {
		return ExitCode.FAILURE;
	}

	// SAFETY: on peut se permettre de déstructurer, car la condition ci-haut
	// permet d'être certain qu'il y a les arguments nécessaires.
	let [ui, level] = argv;

	wasm_initialize_logger(level);

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

main(UI.Vue, LOG_LEVEL).then((code) => {
	if (code === ExitCode.FAILURE) {
		throw new Error("exit failure");
	}

	if (import.meta.env.DEV) {
		console.groupCollapsed("Test");
		dbg("Test");
		info("Test");
		warn("Test");
		error("Test");
		trace("Test");
		console.groupEnd();
	}
})
	.catch(console.error);
