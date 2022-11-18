<script lang="ts" setup>
import type { Room, Server } from "~/types";

import SidebarItem from "~vue/molecules/SidebarItem/SidebarItem.vue";

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
		name: "Private",
		is_active: false,
		is_highlighted: false,
		total_unread_message: 2,
		total_unread_event: 3,
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
];

let server: Server = {
	name: "localhost",
	is_focused: true,
	is_folded: false,
	rooms,
};
</script>

<template>
	<section class="[ flex! gap=4 p=1 ]">
		<SidebarItem v-bind="server.rooms[0]" />
		<SidebarItem v-bind="server.rooms[1]" />
		<SidebarItem v-bind="server.rooms[2]" />
	</section>
</template>
