<script lang="ts">
export default {
	name: "SidebarItemPrivate",
};
</script>

<script lang="ts" setup>
import type { Room } from "~/server";

import { to_user_friendly } from "@phisyrc/std/int/user_friendly";

import IconRoomClose from "~vue/assets/icons/IconCross.vue";
import IconRoomMessageEmpty from "~vue/assets/icons/IconMessageEmpty.vue";
import IconRoomMessage from "~vue/assets/icons/IconMessage.vue";

import Button from "~vue/components/Button.vue";

type Props = {
	name: Room["name"];
	last_message: NonNullable<Room["last_message"]>;
	is_active?: Room["is_active"];
	is_highlighted?: Room["is_highlighted"];
	total_unread_event?: Room["total_unread_event"];
	total_unread_message?: Room["total_unread_message"];
};

const props = defineProps<Props>();
</script>

<template>
	<li
		class="network@server@room"
		data-type="private"
		:class="{
			'app:bg=primary is-active': is_active,
			'app:bg=primary(:hover) is-not-active': !is_active,
			'network@server@room--has-events':
				!is_active && (total_unread_event || 0) > 0,
			'network@server@room--has-messages':
				!is_active && (total_unread_message || 0) > 0,
			'network@server@room--highlight':
				!is_active && is_highlighted && (total_unread_message || 0) > 0,
		}"
	>
		<div>
			<IconRoomMessageEmpty
				v-if="(total_unread_message || total_unread_event) === 0"
			/>
			<IconRoomMessage v-else />
		</div>

		<div>
			{{ name }}
		</div>

		<div class="network@server@room__actions [ flex f:center gap=1 ]">
			<div
				v-if="total_unread_message || total_unread_event"
				class="network@server@room__total-unread-message [ h=3 px=1 align-t:center border:radius=3 ]"
			>
				{{
					to_user_friendly(
						total_unread_message || total_unread_event || 0
					)
				}}
			</div>

			<Button class="network@server@room:close">
				<IconRoomClose />
			</Button>
		</div>

		<br />

		<div class="network@server@room__last-message [ border:radius=1 ]">
			<p class="[ scroll:y my=0 p=1 max-h=7 ]">
				{{ last_message.message }}
			</p>
		</div>
	</li>
</template>
