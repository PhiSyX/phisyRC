<script lang="ts">
export default {
	name: "Window",
};
</script>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

import type { Server } from "~/server";

import Sidebar from "~vue/organisms/Sidebar/Sidebar.vue";

let route = useRoute();

const has_meta_sidebar = route.meta["sidebar"];

type Props = {
	sidebar: boolean;
	servers: Server[];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:sidebar"]);

let sidebar$ = computed({
	get() {
		return props.sidebar;
	},
	set($1: boolean) {
		emit("update:sidebar", $1);
	},
});
</script>

<template>
	<div class="window [ flex size:full border:radius=1 ]">
		<section
			v-if="has_meta_sidebar"
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
