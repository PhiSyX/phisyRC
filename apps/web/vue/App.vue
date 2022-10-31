<script setup lang="ts">
import { computed, ref } from 'vue';
import {
	State,
	useWebSocketStore
} from './stores/websocket';

let websocket = useWebSocketStore(write_output);
let websocket_url = ref("ws://localhost:9667/");

let input = ref("hello world");
let output = ref<[State, ...Array<unknown>][]>([] as any);
let current_state = ref(State.CLOSED);

let ws_connected = computed(() => {
	return [State.CONNECTED, State.RECV, State.SEND]
		.includes(current_state.value);
});

function handle_click_connect(_: MouseEvent) {
	websocket.connect(websocket_url.value);

	websocket.on("open", function () { // FIXME(phisyx): ne fonctionne pas en dehors du scope.
		this.write("CAP LS 302");
		this.write("NICK PhiSyX");
		this.write("USER PhiSyX 8 * :utilisateur webSocket");
	});
}

function handle_click_close(_: MouseEvent) {
	websocket.close();
}

function handle_submit(evt: Event) {
	evt.preventDefault();
	write_socket_from_input();
}

function write_socket_from_input() {
	if (input.value.startsWith("/")) {
		websocket.write(`${input.value.slice(1)}`);
	} else if (input.value.trim().length > 0) {
		websocket.write(`PRIVMSG #room :${input.value}`);
	}
	input.value = "";
}

function write_output(state: State, ...args: Array<unknown>) {
	let item: [State, ...Array<unknown>] = [state, ...args];
	output.value.push(item);
	current_state.value = state;
}
</script>

<template>
	<h1>Test communication WebSocket</h1>

	<form method="post" @submit="handle_submit" class="[ flex! ]">
		<div class="form-group" v-if="!ws_connected">
			<input v-model="websocket_url" type="text">
			<button type="button" @click="handle_click_connect">Se connecter</button>
		</div>
		<div class="form-group" v-else>
			<button type="button" @click="handle_click_close">Fermer la connexion</button>
		</div>

		<div class="history [ f:full fs:center ]">
			<output class="[ mb=1 ]">
				<p v-if="output.length > 0" v-for="[state, item] in output" class="[ m=0 ]">
					<span>[{{ state }}]</span>: {{ item }}
				</p>
			</output>

			<div class="form-group" v-show="ws_connected">
				<input v-model="input" type="text" class="[ flex:full ]">
				<button type="submit">Envoie du message au serveur</button>
			</div>
		</div>
	</form>
</template>

<style scoped lang="scss">
@import "design/functions";
@import "design/mixins";

form {
	height: calc(100% - 68px);
	gap: space(1);
}

.form-group {
	display: flex;
	gap: space(1);
	justify-content: space-between;
}

output {
	overflow: auto;
	display: block;

	width: 80ch;
	height: 50ch;

	border-radius: 4px;

	// NOTE(phisyx): l'historique est vide.
	&:not(:empty) {
		border: 1px ridge var(--color-orange600);
		padding: space(1);
	}

	background-color: var(--default-background);
}

p span {
	color: var(--color-grey);
}

button,
input {
	padding: space(1);
	border-radius: 4px;

	border: 1px solid var(--color-grey800);
}

input {
	transition: all 250ms ease-in-out;

	&:active,
	&:focus {
		outline: 0;
		border: 1px solid var(--color-orange600);
	}

	@include --theme using ($name) {
		@if $name==dark {
			background-color: #3c4043;
		}
	}
}

button {
	background-color: var(--color-white);
	color: var(--color-black);

	&:active {
		background-color: var(--color-grey);
	}
}
</style>
