/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

import { defineConfig } from "vite";

import { phisyRC_execCargoCommandOnChange } from "./vite/plugin";
import vue from "@vitejs/plugin-vue";
import basicSsl from "@vitejs/plugin-basic-ssl";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

import path from "node:path";

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
