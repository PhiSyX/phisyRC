/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import type { Plugin } from "vite";

import { spawn } from "node:child_process";

// @ts-expect-error : chuuut
import pkg from "../package.json" assert { type: "json" };

/// Execute la commande dev:before du package.json.
function phisyRC_execCargoCommandOnChange(): Plugin {
	return {
		name: "phisyRC",
		configureServer(server) {
			server.watcher.on("change", (input) => {
				if (input.includes("generated.scss")) {
					return;
				}

				if (!input.endsWith(".vue")) {
					return;
				}

				let script = pkg["scripts"]["scss:build"].split(' ');
				let cmd = script[0];
				let args = script.slice(1);

				args.push("-f", input);

				spawn(cmd, args, {
					shell: true,
					stdio: "inherit",
					env: process.env
				});
			});
		},
	}
}

export { phisyRC_execCargoCommandOnChange };
