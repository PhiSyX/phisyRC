<script lang="ts">
export default {
	name: "Window",
};
</script>

<template>
	<div class="window [ size:full ]">
		<main role="main" class="window@body [ size:full ]">
			<RouterView />
		</main>
	</div>
</template>
