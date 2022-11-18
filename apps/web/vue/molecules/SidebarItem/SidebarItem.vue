<script lang="ts">
const SidebarItemChannel = defineAsyncComponent(
	() => import("./SidebarItemChannel.vue")
);
const SidebarItemPrivate = defineAsyncComponent(
	() => import("./SidebarItemPrivate.vue")
);

export default {
	name: "SidebarItem",
	components: {
		SidebarItemChannel,
		SidebarItemPrivate,
	},
};
</script>

<script lang="ts" setup>
import type { Room } from "~/server";

import { computed, defineAsyncComponent } from "vue";

import { capitalize } from "@phisyrc/std/str/capitalize";

type Props = {
	type: Room["type"];
};

const props = defineProps<Props>();

const component_item = computed(() => {
	return `SidebarItem${capitalize(props.type)}`;
});
</script>

<template>
	<component
		:is="component_item"
		:type="type"
		v-bind="$attrs"
		class="sidebar@item"
	/>
</template>

<style lang="scss">
@import "design/app/molecules/sidebar-item";
</style>
