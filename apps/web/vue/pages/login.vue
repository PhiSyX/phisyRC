<script lang="ts">
export default {
	name: "Login",
	components: { Waves },
};
</script>

<script setup lang="ts">
import AddChannelIcon from "~vue/assets/icons/Add.vue";
import ChannelIcon from "~vue/assets/icons/Channel.vue";
import CrossIcon from "~vue/assets/icons/Cross.vue";
import PasswordIcon from "~vue/assets/icons/Password.vue";
import TrashDeleteIcon from "~vue/assets/icons/TrashDelete.vue";
import UserIcon from "~vue/assets/icons/User.vue";
import ValidatedIcon from "~vue/assets/icons/Validated.vue";
import VisualPasswordIcon from "~vue/assets/icons/VisualPassword.vue";

import Waves from "~vue/components/Waves.vue";

import { computed, ref } from "vue";

import { MAXLENGTH_NICKNAME, VALIDATION_NICKNAME_INFO } from "constants/login";
import { Option } from "std/option";
import { focus_button_channel, set_selected_channel } from "~/handlers/login";

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

let $channel_list_btn = ref<HTMLElement | null>(null);
let channels = ref<Vec<str>>(["#irc", "#ibug"]);
let selected_channel = ref<Vec<usize>>([]);

function set_selected_channel_handler(evt: MouseEvent, chan_idx: usize) {
	selected_channel.value = set_selected_channel(
		evt,
		selected_channel.value,
		chan_idx
	);
}

function focus_button_channel_handler(evt: MouseEvent | KeyboardEvent) {
	focus_button_channel(evt, Option.from($channel_list_btn.value));
}

function add_channel_handler(evt: Event) {}

function handle_send_connection(evt: Event) {
	evt.preventDefault();

	console.log(nickname.value, password.value);
}
</script>

<template>
	<div id="login-page" class="[ size:full scroll:y scroll:hidden ]">
		<Waves />

		<div class="login@chat<form> [ size:full p=2 flex! gap=6 pos-r ]">
			<h1 class="[ h3-like mx=1 mt=4 mb=0 ]">Accès direct au Chat</h1>

			<form
				:action="form_action_attribute"
				method="post"
				id="login-form"
				class="[ flex! mx=1 px=1 border@radius=4 box:shadow ]"
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

				<div class="[ f:start ]">
					<label for="channels" class="[ t:center ]">
						<ChannelIcon />
					</label>

					<ol
						class="[[ flex:full ] list:reset flex flex:wrap gap=1 ]"
					>
						<li
							v-for="(channel, i) in channels"
							:key="channel"
							:class="[
								'login@channel:label',
								'[ p=1 c:pointer border@radius=2 f=roboto ]',
								{ 'is-selected': selected_channel.includes(i) },
							]"
							@click="set_selected_channel_handler($event, i)"
						>
							{{ channel }}
						</li>

						<li class="[ flex:full ]">
							<input
								type="text"
								class="login@channel:input [ w:full ]"
								name="channels"
								placeholder="Ajouter un ou plusieurs salons"
								@keydown="focus_button_channel_handler"
								@click="focus_button_channel_handler"
							/>
						</li>
					</ol>

					<div class="[ flex! gap=1 t:center ]">
						<button
							id="channels"
							ref="$channel_list_btn"
							type="button"
							@click="add_channel_handler"
						>
							<AddChannelIcon />
						</button>
						<button
							type="button"
							:class="[
								{
									'v:visible': selected_channel.length > 0,
									'v:hidden': selected_channel.length === 0,
								},
							]"
						>
							<TrashDeleteIcon />
						</button>
					</div>
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
	</div>
</template>

<style>
#login-page {
	max-width: 500px;
	align-self: center;
	/*margin: 0 auto; */
}
</style>
