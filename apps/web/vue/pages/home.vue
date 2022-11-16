<script lang="ts">
export default {
	name: "Home",
};
</script>

<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { Server } from "~/server";

import Window from "~vue/window/Window.vue";

let sidebar_opened = ref(false);
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
