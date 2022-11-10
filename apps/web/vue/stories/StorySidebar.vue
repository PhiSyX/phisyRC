<script lang="ts" setup>
import { reactive, ref } from "vue";
import { Room, Server } from "~/server";

import SidebarLayout from "~vue/sidebar/SidebarLayout.vue";

let sidebar1 = ref(false);
let sidebar2 = ref(true);

let rooms: Room[] = [
	{
		name: "#irc",
		type: "channel",
		total_unread_message: 154,
		active: false,
	},
	{
		active: true,
		name: "PhiSyX",
		type: "private",
		total_unread_message: 2,
		last_message: {
			type: "privmsg",
			message:
				"Lorem ipsum dolor sit amet consectetur adipisicing elit. " +
				"Nulla aperiam nihil veniam fugit, eos quidem tempore " +
				"perferendis adipisci architecto! Quia officiis, porro " +
				"doloremque dolorum delectus cupiditate provident corporis " +
				"nemo. Officia!",
			from: {
				nick: "Mike",
			},
		},
	},
	{
		name: "#channel",
		type: "channel",
		total_unread_message: 0,
		active: false,
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
	<section class="[ flex gap=8 ]">
		<div
			class="window@navigation [ resizable:x ]"
			:style="{
				width: sidebar1 ? 'max-content' : 'var(--nav-min-w)',
			}"
		>
			<SidebarLayout v-model:toggle="sidebar1" :servers="servers" />
		</div>

		<div
			class="window@navigation [ resizable:x ]"
			:style="{
				width: sidebar2 ? 'max-content' : 'var(--nav-min-w)',
			}"
		>
			<SidebarLayout v-model:toggle="sidebar2" :servers="servers" />
		</div>
	</section>
</template>

<style lang="scss" scoped>
@import "design/functions";
@import "design/mixins";

div {
	max-width: 320px;
	height: 500px;
}
</style>
