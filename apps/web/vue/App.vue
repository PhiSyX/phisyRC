<script setup lang="ts">
import { ref } from "vue";

let websocket = ref<WebSocket | null>(null);
let websocket_uri = ref("ws://localhost:9667/");

let input = ref("hello world");
let output = ref<any>([]);

enum Output {
	Connected = "Connecté",
	Disconnected = "Déconnecté",
	Error = "Erreur",
	Send = "Envoyé",
	Received = "Reçu",
}

type FIXME = any;

function handle_click_connect(evt: MouseEvent) {
	websocket.value = new WebSocket(websocket_uri.value);
	websocket.value.binaryType = "arraybuffer";

	websocket.value.addEventListener("open", handle_open_connection);
	websocket.value.addEventListener("close", handle_close_connection);
	websocket.value.addEventListener("message", handle_message);
	websocket.value.addEventListener("error", handle_error);
}

function handle_click_close(evt: MouseEvent) {
	handle_close_connection(evt);
}

function handle_submit(evt: Event) {
	evt.preventDefault();
	write_socket_from_input();
}

function handle_open_connection(evt: Event) {
	write_socket("USER PhiSyX 8 * :utilisateur webSocket");
	write_socket("NICK PhiSyX");
	write_output(Output.Connected);
}

function handle_close_connection(evt: CloseEvent | MouseEvent) {
	write_output(Output.Disconnected);
	if (websocket.value) {
		websocket.value!.close();
	}
	websocket.value = null;
}

function handle_error(evt: Event) {
	console.error(evt);
	write_output(Output.Error);
}

function handle_message(evt: MessageEvent) {
	let raw = evt.data;
	if (raw instanceof Blob) {
		let file_reader = new FileReader();
		file_reader.addEventListener("loadend", handle_binary_input);
		file_reader.readAsText(raw);
	} else {
		process(raw);
	}
}

function handle_binary_input(evt: ProgressEvent<FileReader>) {
	let file_reader = evt.target;
	let raw = file_reader?.result! as ArrayBuffer;
	process(raw);
}

function process(raw: NonNullable<ArrayBuffer>) {
	if (!websocket.value) {
		return;
	}
	write_output(Output.Received, raw);
}

function write_socket(message: string) {
	write_output(Output.Send, message);
	let encoder = new TextEncoder();
	let bytes = encoder.encode(`${message}\r\n`);
	websocket.value!.send(bytes);
}

function write_socket_from_input() {
	write_socket(`PRIVMSG #room :${input.value}`);
	input.value = "";
}

function write_output(state: Output, ...args: FIXME) {
	let item = [state, ...args];
	output.value.push(item);
}
</script>

<template>
	<h1>Test communication WebSocket</h1>

	<form method="post" @submit="handle_submit">
		<div class="form-group" v-if="!websocket">
			<input v-model="websocket_uri" type="text">
			<button type="button" @click.once="handle_click_connect">Se connecter</button>
		</div>
		<div class="form-group" v-else>
			<button type="button" @click.once="handle_click_close">Fermer la connexion</button>
		</div>

		<output>
			<p if="output.length > 0" v-for="[state, item] in output">[ {{ state }} ]: {{ item }}</p>
		</output>

		<div class="form-group" v-if="websocket">
			<input v-model="input" type="text">
			<button type="submit">Envoie du message au serveur</button>
		</div>
	</form>
</template>
