/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// @ts-check

import { exec as child_process_exec } from "node:child_process";
import { existsSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import { promisify } from "node:util";

const CARGO_BINARY = "generate-style";

const PREFIX_FORMAT = "\x1b[";
const SUFFIX_FORMAT = "m";
const NEWLINE = process.platform.includes("win") ? "\r\n" : "\n";

const CYAN = 36;
const RED = 101;
const UNDERLINE = 4;


const exec = promisify(child_process_exec);

/**
 * @param {string} text
 * @param {number} f0rmat
 * @return {string}
 */
function format(text, f0rmat) {
	let pre = PREFIX_FORMAT;
	let suf = SUFFIX_FORMAT;
	return `${pre}${f0rmat}${suf}${text}${pre}${suf}`;
}

/**
 * @param {string} text
 * @param {number} color
 * @return {string}
 */
function colorize(text, color) {
	return format(text, color);
}

/**
 * @param {string} text
 * @param {number} style
 * @return {string}
 */
function stylize(text, style) {
	return format(text, style);
}

/**
 * @param {ArgumentFlagException | import("node:child_process").ExecException} err
 */
function error(err) {
	let err_r = colorize("error", RED);
	let dir_r = stylize(basename(dirname(import.meta.url)), UNDERLINE);
	let bin_r = stylize(CARGO_BINARY, UNDERLINE);
	console.error(`${err_r}(${dir_r}/${bin_r}):`, err);
	return err;
}

class ArgumentFlagException extends Error {
	constructor() {
		let flag = colorize("-t <target-filepath>", CYAN);
		super(`le drapeau \`${flag}\` est obligatoire.${NEWLINE}`);
	}
}

// ---- //
// Main //
// ---- //

function main() {
	let args = process.argv.slice(2);

	if (args.length < 2) {
		return error(new ArgumentFlagException());
	}

	let binary_name = CARGO_BINARY;
	if (process.platform.includes("win")) {
		binary_name += ".exe";
	}

	let binary_file = resolve("..", "..", "target", "release", binary_name);
	// EXAMPLE: -t <target-filepath>
	let binary_args = ["-t", args[1]];

	// EXAMPLE: -t <target-filepath> -f <html-filepath>
	if (args.length === 4) {
		binary_args.push("-f", args[3]);
	}

	if (existsSync(binary_file)) {
		exec(`${binary_file} ${binary_args.join(" ")}`).catch(error);
	} else {
		// Exécute la commande `cargo run` si le binaire n'existe pas, qui
		// s'occupe de créer le binaire in fine.
		exec(`cargo run --release --bin ${CARGO_BINARY} -- ${binary_args.join(" ")}`).catch(error);
	}
}

main();
