<script lang="ts">
export default {
	name: "SidebarList",
};
</script>

<script lang="ts" setup>
import type { Props as SidebarListProps } from "~/molecules/SidebarList/props";

import { computed } from "vue";

import SidebarItem from "~vue/molecules/SidebarItem/SidebarItem.vue";
import SidebarItemServer from "~vue/molecules/SidebarItem/SidebarItemServer.vue";

type Props = {
	name: SidebarListProps["name"];
	rooms: SidebarListProps["rooms"];
	is_focused: SidebarListProps["is_focused"];
	is_folded: SidebarListProps["is_folded"];

	// NOTE(phisyx): v-model:folded
	folded: SidebarListProps["is_folded"];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:folded"]);

let folded$ = computed({
	get() {
		return props.is_folded;
	},
	set($1: boolean) {
		emit("update:folded", $1);
	},
});
</script>

<template>
	<ul
		class="app:bg=secondary sidebar@list [ list:reset ]"
		:class="{
			'is-folded': is_folded,
			'is-not-folded': !is_folded,
			'is-focused': is_focused,
			'is-not-focused': !is_focused,
		}"
	>
		<SidebarItemServer
			:key="name"
			v-bind="props"
			v-model:folded="folded$"
		/>

		<SidebarItem
			v-for="room in rooms"
			:key="name + ':' + room.name"
			v-bind="room"
		/>
	</ul>
</template>

<style lang="scss">
@import "design/app/molecules/sidebar-list";
</style>
