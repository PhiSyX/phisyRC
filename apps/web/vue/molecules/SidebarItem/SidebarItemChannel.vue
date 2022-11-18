<script lang="ts">
export default {
	name: "SidebarItemChannel",
};
</script>

<script lang="ts" setup>
import type { ChannelProps as SidebarItemChannelProps } from "~/molecules/SidebarItem/props";

import { to_user_friendly } from "@phisyrc/std/int/user_friendly";

import IconRoomClose from "~vue/atoms/Icons/IconCross.vue";
import IconRoomMessageEmpty from "~vue/atoms/Icons/IconMessageEmpty.vue";
import IconRoomMessage from "~vue/atoms/Icons/IconMessage.vue";

import Button from "~vue/atoms/Button/Button.vue";

type Props = {
	name: SidebarItemChannelProps["name"];
	type: SidebarItemChannelProps["type"];
	is_active?: SidebarItemChannelProps["is_active"];
	is_highlighted?: SidebarItemChannelProps["is_highlighted"];
	total_unread_event?: SidebarItemChannelProps["total_unread_event"];
	total_unread_message?: SidebarItemChannelProps["total_unread_message"];
};

const props = defineProps<Props>();
</script>

<template>
	<li
		data-type="channel"
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
	</li>
</template>
