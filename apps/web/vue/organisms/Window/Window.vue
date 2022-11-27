<script lang="ts">
export default {
	name: "Window",
};
</script>

<script setup lang="ts">
import Sidebar from "~vue/organisms/Sidebar/Sidebar.vue";

import type { Props as WindowProps } from "~/organisms/Window/props";
import { useRoute } from "vue-router";
import { use_model } from "~vue/hooks/use_models";

let route = useRoute();

// NOTE(phisyx): Définie dans les meta-données des routes
const has_meta_sidebar = route.meta["sidebar"];

type Props = {
	servers: WindowProps["servers"];

	// NOTE(phisyx): v-model:sidebar
	sidebar: WindowProps["is_sidebar_opened"];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:sidebar"]);

let sidebar$ = use_model(props, "sidebar")(emit);
</script>

<template>
	<div class="window [ flex size:full border:radius=1 ]">
		<section
			v-if="has_meta_sidebar && servers.length"
			class="window@navigation [ resizable:x ]"
			:class="{
				'is-opened': sidebar,
				'is-collapsed': !sidebar,
			}"
			:style="{
				width: sidebar ? 'max-content' : 'var(--nav-min-w)',
			}"
		>
			<Sidebar v-model:toggle="sidebar$" :servers="servers" />
		</section>

		<main
			role="main"
			class="window@body [[ flex:full ] scroll:x flex! border:radius=2 box:shadow ][ p=1 ]"
		>
			<RouterView />
		</main>
	</div>
</template>
