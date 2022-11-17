<script lang="ts">
export default {
	name: "SidebarList",
};
</script>

<script lang="ts" setup>
import type { Server } from "~/server";

import { computed } from "vue";

import SidebarItem from "~vue/molecules/SidebarItem/SidebarItem.vue";
import SidebarItemServer from "~vue/molecules/SidebarItem/SidebarItemServer.vue";

type Props = {
	name: Server["name"];
	rooms: Server["rooms"];
	is_focused: Server["is_focused"];
	is_folded: Server["is_folded"];
	folded: Server["is_folded"];
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
		class="app:bg=secondary network<server> [ list:reset ]"
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
