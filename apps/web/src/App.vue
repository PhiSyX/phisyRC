<script setup lang="ts">
import { ref } from "vue";

let websocket!: WebSocket;
let websocket_uri_el = ref<HTMLInputElement | null>(null);

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

function handle_connect(evt: MouseEvent) {
	let websocket_uri = websocket_uri_el.value!.value;
	websocket = new WebSocket(websocket_uri);
	websocket.binaryType = "arraybuffer";

	websocket.addEventListener("open", handle_open_connection);
	websocket.addEventListener("close", handle_close_connection);
	websocket.addEventListener("message", handle_message);
	websocket.addEventListener("error", handle_error);
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

function handle_close_connection(evt: CloseEvent) {
	write_output(Output.Disconnected);
	websocket!.close();
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
	if (!websocket) {
		return;
	}
	write_output(Output.Received, raw);
}

function write_socket(message: string) {
	write_output(Output.Send, message);
	let encoder = new TextEncoder();
	let bytes = encoder.encode(`${message}\r\n`);
	websocket!.send(bytes);
}

function write_socket_from_input() {
	write_socket(`PRIVMSG #room ${input.value}`);
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
		<div class="form-group">
			<input ref="websocket_uri_el" type="text" value="ws://localhost:9667/">
			<button type="button" @click.once="handle_connect">Se connecter</button>
		</div>

		<output>
			<p if="output.length > 0" v-for="[state, item] in output">[ {{ state }} ]: {{ item }}</p>
		</output>

		<div class="form-group">
			<input v-model="input" type="text">
			<button type="submit">Envoie du message au serveur</button>
		</div>
	</form>
</template>
