/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

import type { Plugin } from "vite";

import { exec } from "node:child_process";
import { defineConfig } from "vite";

// @ts-expect-error : chuuuut
import pkg from "./package.json" assert { type: "json" };

import vue from "@vitejs/plugin-vue";
import basicSsl from "@vitejs/plugin-basic-ssl";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

import path from "node:path";

/// Execute la commande dev:before du package.json.
function phisyRC_execCargoCommandOnChange(): Plugin {
	return {
		name: "phisyRC",
		configureServer(server) {
			server.watcher.on("change", (input) => {
				if (input.includes("generated.scss")) {
					return;
				}

				let exec_cb = (err: unknown) => {
					if (err) {
						console.error(`exec error: ${err}`);
						server.close();
						return;
					}
				};

				exec(pkg["scripts"]["scss:build"], exec_cb);
			});
		},
	}
};

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		basicSsl(),
		phisyRC_execCargoCommandOnChange(),
		topLevelAwait(),
		wasm(),
		vue()
	],

	server: {
		https: true,
	},

	resolve: {
		alias: [
			{ find: /^~vue/, replacement: path.resolve("vue") },
			{ find: /^~/, replacement: path.resolve("src") },
			{ find: /^assets\/wasm/, replacement: path.resolve("assets", "wasm") },
			{ find: /^assets/, replacement: path.resolve("..", "..", "assets") },
			{ find: /^constants/, replacement: path.resolve("constants") },
			{ find: /^design/, replacement: path.resolve("design") },
		],
	},
});
