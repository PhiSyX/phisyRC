<script setup lang="ts">
import { ref } from 'vue';
import { State, useWebSocketStore } from './stores/websocket';

let websocket = useWebSocketStore(write_output);
let websocket_url = ref("ws://localhost:9667/");

let input = ref("hello world");

let output = ref<[State, ...Array<unknown>][]>([] as any);

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
	} else {
		websocket.write(`PRIVMSG #room :${input.value}`);
	}
	input.value = "";
}

function write_output(state: State, ...args: Array<unknown>) {
	let item: [State, ...Array<unknown>] = [state, ...args];
	output.value.push(item);
}
</script>

<template>
	<h1>Test communication WebSocket</h1>

	<form method="post" @submit="handle_submit">
		<div class="form-group" v-if="!websocket.state.connection">
			<input v-model="websocket_url" type="text">
			<button type="button" @click.once="handle_click_connect">Se connecter</button>
		</div>
		<div class="form-group" v-else>
			<button type="button" @click.once="handle_click_close">Fermer la connexion</button>
		</div>

		<div class="history">
			<output>
				<p if="output.length > 0" v-for="[state, item] in output">
					<span>[{{ state }}]</span>: {{ item }}
				</p>
			</output>

			<div class="form-group" v-show="websocket.state.connection">
				<input v-model="input" type="text">
				<button type="submit">Envoie du message au serveur</button>
			</div>
		</div>
	</form>
</template>

<style scoped lang="scss">
form {
	height: calc(100% - 68px);
	display: flex;
	flex-direction: column;
	gap: var(--space);
}

.history {
	align-self: center;
	flex-grow: 1;
}

.form-group {
	display: flex;
	gap: var(--space);
	justify-content: space-between;
}

output {
	overflow: auto;
	display: block;

	width: 80ch;
	height: 50ch;

	border-radius: 4px;
	margin-bottom: var(--space);

	&:not(:empty) {
		border: 1px ridge var(--color-orange600);
		padding: var(--space);
	}

	background-color: var(--default-background);
}

p {
	margin: 0;

}

p span {
	color: var(--color-grey);
}

button,
input {
	padding: var(--space);
	border-radius: 4px;

	border: 1px solid var(--color-grey800);
}

.history input {
	flex-grow: 1;
}

input {
	transition: all 250ms ease-in-out;

	&:active,
	&:focus {
		outline: 0;
	}


	@media (prefers-color-scheme: dark) {
		background-color: #3c4043;

		&:active,
		&:focus {
			border: 1px solid var(--color-orange600);
		}
	}

	@media (prefers-color-scheme: light) {

		&:active,
		&:focus {
			border: 1px solid var(--color-orange600);
		}
	}
}

button {
	background-color: var(--color-white);
	color: var(--color-black);
}
</style>
