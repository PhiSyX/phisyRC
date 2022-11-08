<script lang="ts">
export default {
	name: "Window",
};
</script>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

import SidebarLayout from "~vue/sidebar/SidebarLayout.vue";

let route = useRoute();

const has_meta_sidebar = route.meta["sidebar"];

type Props = {
	sidebar: boolean;
};

const props = defineProps<Props>();

let emit = defineEmits(["update:sidebar"]);

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
			class="window@navigation [ scroll:y scroll:hidden ]"
			:class="{
				'is-opened': sidebar,
				'is-collapsed': !sidebar,
			}"
			:style="{
				// width: sidebar ? 'max-content' : 'var(--nav-min-w)',
				width: sidebar ? '500px' : 'var(--nav-min-w)',
			}"
		>
			<SidebarLayout v-model:toggle="sidebar$" />
		</section>

		<main
			role="main"
			class="window@body [[ flex:full ] scroll:x flex! border:radius=2 box:shadow ][ p=1 ]"
		>
			<RouterView />
		</main>
	</div>
</template>
