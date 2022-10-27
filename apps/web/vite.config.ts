import { defineConfig } from "vite";

import vue from "@vitejs/plugin-vue";

import path from "node:path";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [vue()],

	resolve: {
		alias: [
			{ find: /^~vue/, replacement: path.resolve("vue") },
			{ find: /^~/, replacement: path.resolve("src") },
			{ find: /^design/, replacement: path.resolve("design") },
			{ find: /^std/, replacement: path.resolve("std") },
		],
	},
});
