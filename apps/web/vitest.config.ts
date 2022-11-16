/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

import { defineConfig } from "vitest/config";

import path from "node:path";

// https://vitejs.dev/config/
export default defineConfig({
	resolve: {
		alias: [
			{ find: /^~/, replacement: path.resolve("src") },
		],
	},

	test: {
		includeSource: ["src/**/*.ts"],
	},
});
