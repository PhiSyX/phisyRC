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

import { computed, ref } from "vue";

import type { Server } from "~/server";

type Props = {
	name: Server["name"];
	rooms: Server["rooms"];
	is_focused: Server["is_focused"];
	is_folded: Server["is_folded"];
	folded: Server["is_folded"];
};

const props = defineProps<Props>();

let emit = defineEmits(["update:folded"]);

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
		<li class="app:bg=primary" data-type="server">
			<div>
				<IconServerConnect />
			</div>

			<div class="[ align-t:center ]">{{ name }}</div>

			<div class="network@server__actions">
				<Button v-model:toggle="folded$">
					<IconArrowRight v-if="is_folded" />
					<IconArrowDown v-else />
				</Button>
			</div>
		</li>

		<li
			class="network@server@room"
			v-for="room in rooms"
			:class="{
				'app:bg=primary is-active': room.active,
				'app:bg=primary(:hover) is-not-active': !room.active,
				'network@server@room--has-events':
					!room.active && room.total_unread_event > 0,
				'network@server@room--has-messages':
					!room.active && room.total_unread_message > 0,
				'network@server@room--highlight':
					!room.active &&
					room.highlight &&
					room.total_unread_message > 0,
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

			<br v-if="room.type == 'private'" />

			<div
				class="network@server@room__last-message [ border:radius=1 ]"
				v-if="room.last_message"
			>
				<p class="[ scroll:y my=0 p=1 max-h=7 ]">
					{{ room.last_message?.message }}
				</p>
			</div>
		</li>
	</ul>
</template>
