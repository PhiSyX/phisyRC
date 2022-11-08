<script lang="ts">
export default {
	name: "SidebarItem",
};
</script>

<script lang="ts" setup>
const props = defineProps(["name", "type"]);
</script>

<template>
	<li class="sidebar__item" :data-type="type">
		<div><slot /></div>
	</li>
</template>
