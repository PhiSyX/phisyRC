<script lang="ts">
export default {
	name: "Window",
};
</script>

<template>
	<div class="window [ flex size:full border@radius=1 ]">
		<main
			role="main"
			class="window@body [[ flex:full ] scroll:x flex! border@radius=2 ][ p=1 ]"
		>
			<RouterView />
		</main>
	</div>
</template>
