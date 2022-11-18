<script lang="ts" setup>
import { reactive, ref } from "vue";

import type { Room, Server } from "~/types";

import SidebarList from "./SidebarList.vue";

let sidebar1 = ref(false);

let rooms: Room[] = [
	{
		type: "channel",
		name: "#channel (message)",
		is_active: false,
		is_highlighted: false,
		total_unread_message: 1542,
		total_unread_event: 2,
	},
	{
		type: "private",
		name: "Private (active room)",
		is_active: true,
		is_highlighted: false,
		total_unread_message: 2,
		total_unread_event: 0,
		last_message: {
			type: "privmsg",
			message:
				"Lorem ipsum dolor sit amet consectetur adipisicing elit. " +
				"Nulla aperiam nihil veniam fugit, eos quidem tempore " +
				"perferendis adipisci architecto! Quia officiis, porro " +
				"doloremque dolorum delectus cupiditate provident corporis " +
				"nemo. Officia!",
			from: {
				nick: "Private",
			},
		},
	},
	{
		type: "channel",
		name: "#channel (event)",
		is_active: false,
		is_highlighted: false,
		total_unread_event: 1,
		total_unread_message: 0,
	},
	{
		type: "private",
		name: "Private (highlight)",
		is_active: false,
		is_highlighted: true,
		total_unread_event: 0,
		total_unread_message: 1,
		last_message: {
			type: "privmsg",
			message: "Bonjour Private.",
			from: {
				nick: "Private",
			},
		},
	},
];

let servers: Server[] = reactive([
	{
		name: "localhost",
		is_focused: true,
		is_folded: false,
		rooms,
	},
]);
</script>

<template>
	<section class="[ flex gap=4 p=1 ]">
		<SidebarList v-model:folded="sidebar1" v-bind="servers[0]" />
	</section>
</template>
