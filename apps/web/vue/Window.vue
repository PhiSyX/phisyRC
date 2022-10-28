<script setup lang="ts">
import { useRoute } from "vue-router";

const route = useRoute();
</script>

<template>
	<div class="window">
		<main role="main" class="window@body">
			<RouterView />
		</main>
	</div>
</template>
