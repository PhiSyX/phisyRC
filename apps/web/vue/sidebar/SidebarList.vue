<script lang="ts">
export default {
	name: "SidebarList",
};
</script>

<script lang="ts" setup>
import IconArrowDown from "~vue/assets/icons/IconArrowDown.vue";
import IconArrowRight from "~vue/assets/icons/IconArrowRight.vue";
import IconRoomClose from "~vue/assets/icons/IconCross.vue";
import IconServerConnect from "~vue/assets/icons/IconServerConnect.vue";
import IconRoomMessageEmpty from "~vue/assets/icons/IconMessageEmpty.vue";
import IconRoomMessage from "~vue/assets/icons/IconMessage.vue";

import Button from "~vue/components/Button.vue";

import { ref } from "vue";

import type { Server } from "~/server";

type Props = {
	name: Server["name"];
	rooms: Server["rooms"];
};

const { name, rooms } = defineProps<Props>();

const folded = ref(false);
</script>

<template>
	<li class="app:bg=primary" data-type="server">
		<div>
			<IconServerConnect />
		</div>

		<div class="[ align-t:center ]">{{ name }}</div>

		<div class="network@server__actions">
			<Button v-model:toggle="folded">
				<IconArrowRight v-if="folded" />
				<IconArrowDown v-else />
			</Button>
		</div>
	</li>

	<li
		v-show="!folded"
		class="network@server@room app:bg=primary(:hover)"
		v-for="room in rooms"
		:class="{
			'is-active': room.active,
			'is-not-active': !room.active,
		}"
		:data-type="room.type"
	>
		<div>
			<IconRoomMessageEmpty v-if="room.last_message == null" />
			<IconRoomMessage v-else />
		</div>

		<div>
			{{ room.name }}
		</div>

		<div class="network@server@room__actions [ flex f:center gap=1 ]">
			<div
				v-if="room.total_unread_message"
				class="network@server@room__total-unread-message [ h=3 px=1 align-t:center border:radius=3 ]"
			>
				{{ room.total_unread_message }}
			</div>

			<Button class="network@server@room:close">
				<IconRoomClose />
			</Button>
		</div>

		<br />

		<div
			class="network@server@room__last-message [ border:radius=1 ]"
			v-if="room.last_message"
		>
			<p class="[ scroll:y my=0 p=1 max-h=7 ]">
				{{ room.last_message?.message }}
			</p>
		</div>
	</li>
</template>
