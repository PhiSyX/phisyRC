/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import { exec } from "node:child_process";
import { existsSync } from "node:fs";
import { resolve } from "node:path";

const CARGO_BINARY = "generate-style";

let args = process.argv.slice(2);

let binary_name = CARGO_BINARY;
if (process.platform.includes("win")) {
	binary_name += ".exe";
}

let binary_file = resolve("..", "..", "target", "release", binary_name);
let binary_args = [
	"-t",
	args[1],
	...(args.length === 4 ? ["-f", args[3]] : []),
];

const exec_cb = (err) => {
	if (err) {
		console.error(`exec error: ${err}`);
		return;
	}
};

if (existsSync(binary_file)) {
	exec(`${binary_file} ${binary_args.join(" ")}`, exec_cb);
} else {
	// Exécute la commande `cargo run` si le binaire n'existe pas.
	exec(
		`cargo run --release --bin ${CARGO_BINARY} -- ${binary_args.join(" ")}`,
		exec_cb,
	);
}
