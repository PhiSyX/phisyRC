<script lang="ts">
export default {
	name: "Login",
};
</script>

<script setup lang="ts">
import CrossIcon from "~vue/assets/icons/Cross.vue";
import PasswordIcon from "~vue/assets/icons/Password.vue";
import UserIcon from "~vue/assets/icons/User.vue";
import ValidatedIcon from "~vue/assets/icons/Validated.vue";
import VisualPasswordIcon from "~vue/assets/icons/VisualPassword.vue";

import { computed, ref } from "vue";

import { MAXLENGTH_NICKNAME, VALIDATION_NICKNAME_INFO } from "constants/login";
import { Option } from "std/option";

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

function handle_send_connection(evt: Event) {
	evt.preventDefault();

	console.log(nickname.value, password.value);

	// if (nickname.value.trim().length === 0) {
	// 	has_nickname_error.value = true;
	// 	return;
	// }

	// if (password.value) {
	// 	let COMMAND_PASS = computed(() => {
	// 		return `PASS ${password.value}`;
	// 	});
	// 	websocket?.write(COMMAND_PASS.value);
	// }

	// let COMMAND_NICK = computed(() => {
	// 	return `NICK ${nickname.value}`;
	// });

	// let COMMAND_USER = computed(() => {
	// 	return `USER ${nickname.value} 8 * :utilisateur websocket`;
	// });

	// websocket?.write(COMMAND_NICK.value);
	// websocket?.write(COMMAND_USER.value);
}
</script>

<template>
	<div id="login-page" class="login@chat<form> [ size:full p=2 flex! gap=2 ]">
		<h1 class="[ h3-like m=5 mb=0 ]">Accès direct au Chat</h1>

		<form
			:action="form_action_attribute"
			method="post"
			id="login-form"
			class="[ flex! mx=1 px=1 border@radius=4 ]"
			@submit="handle_send_connection"
		>
			<div class="[ f:center ]">
				<label for="nickname" class="[ t:center ]">
					<UserIcon />
				</label>

				<input
					id="nickname"
					name="nickname"
					type="text"
					required
					:maxlength="MAXLENGTH_NICKNAME"
					:placeholder="`Pseudonyme (max. ${MAXLENGTH_NICKNAME} caractères)`"
					:title="VALIDATION_NICKNAME_INFO"
					v-model="nickname"
				/>

				<span
					class="[ t:center ]"
					v-if="has_nickname_error !== undefined"
				>
					<CrossIcon v-if="has_nickname_error === true" />
					<ValidatedIcon v-if="has_nickname_error === false" />
				</span>
			</div>

			<div class="[ f:center ]">
				<label for="server_password" class="[ t:center ]">
					<PasswordIcon />
				</label>

				<input
					id="server_password"
					autocomplete="off"
					name="server_password"
					placeholder="Mot de passe serveur (optionnel)"
					:type="server_password_input_type"
					v-model="password"
				/>

				<span
					class="[ t:center ]"
					title="Voir le mot de passe"
					@click="handle_toggle_visual_password"
					:style="{
						opacity: can_show_server_password ? 0.5 : 1,
					}"
				>
					<VisualPasswordIcon />
				</span>
			</div>
		</form>

		<button
			type="submit"
			form="login-form"
			class="[ w:full p=2 border@radius=1 ]"
		>
			Accéder au Chat
		</button>
	</div>
</template>

<style>
#login-page {
	max-width: 500px;
	margin: 0 auto;
}
</style>
