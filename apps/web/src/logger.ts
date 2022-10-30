/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import {
	logger_debug,
	logger_error,
	logger_info,
	logger_trace,
	logger_warn,
} from "assets/wasm/phisyrc_wasm";

// ----------- //
// Énumération //
// ----------- //

const enum LogLevel {
	Error = "ERROR",
	Warn = "WARN",
	Info = "INFO",
	Debug = "DEBUG",
	Trace = "TRACE",
}

// -------- //
// Fonction //
// -------- //

function error<T extends unknown[]>(...args: T): T {
	logger_error(args);
	return args;
}

function info<T extends unknown[]>(...args: T): T {
	logger_info(args);
	return args;
}

function dbg<T extends unknown[]>(...args: T): T {
	if (!import.meta.env.DEV) {
		warn(
			"Utilisation de la fonction `dbg!()` dans un environnement dans un" +
				"environnement autre que `DEV`.",
		);
		return args;
	}

	logger_debug(args);
	return args;
}

function trace<T extends unknown[]>(...args: T): T {
	logger_trace(args);
	return args;
}

function warn<T extends unknown[]>(...args: T): T {
	logger_warn(args);
	return args;
}

// ------ //
// Export //
// ------ //

export { dbg, error, info, trace, warn };

export { LogLevel };
