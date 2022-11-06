<script lang="ts">
export default {
	name: "Home",
};
</script>

<script setup lang="ts">
import { onMounted } from "vue";
import Window from "~vue/Window.vue";

onMounted(() => {
	document.documentElement.dataset["js"] = "on";
});
</script>

<template>
	<div class="app [ size:full ]">
		<Window />
	</div>
</template>
