import { defineConfig } from "npm:vite";
import vue from "npm:@vitejs/plugin-vue";

import "npm:vue";
import "npm:pinia";
import "npm:vue-router";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [vue()],

	resolve: {
		alias: [
			{ find: /^~vue/, replacement: "../vue" },
			{ find: /^~/, replacement: "../src" },
			{ find: /^design/, replacement: "../design" },
			{ find: /^std/, replacement: "../std" },
		],
	},
});
