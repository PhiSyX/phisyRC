/// <reference path="../node_modules/vite/client.d.ts" />

declare module "*.vue" {
	import type { DefineComponent } from "vue";
	const component: DefineComponent<{}, {}, any>;
	export default component;
}
