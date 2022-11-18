<script lang="ts">
export default {
	name: "Home",
};
</script>

<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";

import type { Server } from "~/types";

import Window from "~vue/organisms/Window/Window.vue";

let sidebar_opened = ref(false);
// TODO(phisyx): serveurs récupérés depuis un état par exemple...
let servers: Server[] = reactive([]);

onMounted(() => {
	document.documentElement.dataset["js"] = "on";
});
</script>

<template>
	<div class="app [ size:full p=1 ]">
		<Window v-model:sidebar="sidebar_opened" :servers="servers" />
	</div>
</template>
