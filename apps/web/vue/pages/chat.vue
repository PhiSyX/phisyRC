<script lang="ts">
export default {
	name: "Chat",
};
</script>

<script setup lang="ts">
import App from "~vue/App.vue"
</script>

<template>
	<div id="chat-page">
		<App />
	</div>
</template>

<style scoped>
#chat-page {
	width: 100%;
	height: 100%;
	padding: calc(var(--space) * 2);
}
</style>
