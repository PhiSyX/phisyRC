<script lang="ts" setup>
import IconAddChannel from "~vue/atoms/Icons/IconAdd.vue";
import IconChannel from "~vue/atoms/Icons/IconChannel.vue";
import IconCross from "~vue/atoms/Icons/IconCross.vue";
import IconPassword from "~vue/atoms/Icons/IconPassword.vue";
import IconTrashDelete from "~vue/atoms/Icons/IconTrashDelete.vue";
import IconUser from "~vue/atoms/Icons/IconUser.vue";
import IconValidated from "~vue/atoms/Icons/IconValidated.vue";
import IconVisualPassword from "~vue/atoms/Icons/IconVisualPassword.vue";

import Button from "~vue/atoms/Button/Button.vue";
import Input from "~vue/atoms/Input/Input.vue";

import { computed, ref } from "vue";

import {
	MAXLENGTH_NICKNAME,
	PLACEHOLDER_NICKNAME,
	PLACEHOLDER_SERVER_PASSWORD,
	PLACEHOLDER_CHANNELS,
	VALIDATION_NICKNAME_INFO,
	BYPASS_CONFIRM_DELETE_CHANNEL,
} from "~/organisms/LoginForm/constant";
import { Option } from "@phisyrc/std/option";
import {
	focus_button_channel,
	set_selected_channel,
	unset_selected_channel,
} from "~/organisms/LoginForm/handler";

let form_action_attribute = Option.from(
	import.meta.env.VITE_PHISYRC_LOGIN_CHAT_URL
).unwrap_or("#");

// -------- //
// Nickname //
// -------- //

let nickname = ref("");
let has_nickname_error = ref();

// -------- //
// Password //
// -------- //

let password = ref("");

let can_show_server_password = ref(false);
let server_password_input_type = computed(() => {
	if (can_show_server_password.value) {
		return "text";
	}
	return "password";
});

function handle_toggle_visual_password() {
	can_show_server_password.value = !can_show_server_password.value;
}

// -------- //
// Channels //
// -------- //

let $channel_list_btn = ref<typeof Button | null>(null);
let channels = ref<Vec<str>>(["#irc", "#ibug"]);
let selected_channel = ref<Vec<usize>>([]);

// ------- //
// Handler //
// ------- //

function set_selected_channel_handler(evt: MouseEvent, chan_idx: usize) {
	selected_channel.value = set_selected_channel(
		evt,
		selected_channel.value,
		chan_idx
	);
}

function unset_selected_channel_handler(evt: MouseEvent) {
	channels.value = unset_selected_channel(
		evt,
		channels.value,
		selected_channel.value
	);
}

function focus_button_channel_handler(evt: MouseEvent | KeyboardEvent) {
	focus_button_channel(
		evt,
		Option.from($channel_list_btn.value).map((v) => v.$el)
	);
}

function add_channel_handler(evt: Event) {}

function handle_send_connection(evt: Event) {
	evt.preventDefault();

	console.log(nickname.value, password.value);
}
</script>

<template>
	<form
		:action="form_action_attribute"
		method="post"
		class="login@form [ flex! mx=1 px=1 border:radius=4 box:shadow ]"
		@submit="handle_send_connection"
	>
		<Input
			rclass="[ align-i:center ]"
			lclass="[ align-t:center ]"
			iclass="[ align-t:center ]"
			name="nickname"
			type="text"
			required
			:maxlength="MAXLENGTH_NICKNAME"
			:placeholder="PLACEHOLDER_NICKNAME"
			:title="VALIDATION_NICKNAME_INFO"
			v-model="nickname"
		>
			<IconUser />

			<template #icon v-if="has_nickname_error != undefined">
				<IconCross v-if="has_nickname_error == true" />
				<IconValidated v-if="has_nickname_error == false" />
			</template>
		</Input>

		<Input
			rclass="[ align-i:center ]"
			lclass="[ align-t:center ]"
			iclass="[ align-t:center cursor:pointer ]"
			name="server_password"
			autocomplete="off"
			:placeholder="PLACEHOLDER_SERVER_PASSWORD"
			:type="server_password_input_type"
			v-model="password"
		>
			<IconPassword />

			<template #icon>
				<IconVisualPassword
					@click="handle_toggle_visual_password"
					:style="{
						opacity: can_show_server_password ? 0.5 : 1,
					}"
				/>
			</template>
		</Input>

		<Input
			rclass="[ align-t:center ]"
			lclass="[ align-t:center ]"
			iclass="[ align-t:center flex! gap=1 ]"
			dclass="[ flex:full ]"
			:diclass="
				(i) => [
					'login@channel:label',
					'[ p=1 cursor:pointer border:radius=2 f-family=roboto ]',
					{ 'is-selected': selected_channel.includes(i) },
				]
			"
			:diclick="set_selected_channel_handler"
			name="channels"
			:datalist="channels"
			:placeholder="PLACEHOLDER_CHANNELS"
			@keydown="focus_button_channel_handler"
			@click="focus_button_channel_handler"
		>
			<IconChannel />

			<template #icon>
				<Button ref="$channel_list_btn" @click="add_channel_handler">
					<IconAddChannel />
				</Button>

				<Button
					:title="BYPASS_CONFIRM_DELETE_CHANNEL"
					:class="[
						{
							'v:visible': selected_channel.length > 0,
							'v:hidden': selected_channel.length === 0,
						},
					]"
					@click="unset_selected_channel_handler"
				>
					<IconTrashDelete />
				</Button>
			</template>
		</Input>
	</form>
</template>

<style lang="scss">
@import "design/app/organisms/login-form";
</style>