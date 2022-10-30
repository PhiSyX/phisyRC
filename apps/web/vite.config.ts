/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

import { defineConfig } from "vite";

import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

import path from "node:path";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [topLevelAwait(), wasm(), vue()],

	resolve: {
		alias: [
			{ find: /^~vue/, replacement: path.resolve("vue") },
			{ find: /^~/, replacement: path.resolve("src") },
			{ find: /^assets/, replacement: path.resolve("assets") },
			{ find: /^design/, replacement: path.resolve("design") },
			{ find: /^std/, replacement: path.resolve("std") },
		],
	},
});
