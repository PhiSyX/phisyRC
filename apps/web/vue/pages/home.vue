<script lang="ts">
export default {
	name: "Home",
};
</script>

<script setup lang="ts">
import { onMounted, ref } from "vue";

import Window from "~vue/window/Window.vue";

let sidebar_opened = ref(false);

onMounted(() => {
	document.documentElement.dataset["js"] = "on";
});
</script>

<template>
	<div class="app [ size:full p=1 ]">
		<Window v-model:sidebar="sidebar_opened" />
	</div>
</template>
