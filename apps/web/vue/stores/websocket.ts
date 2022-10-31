/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import { reactive, ref } from "vue";

const enum State {
	CLOSED = "Déconnecté",
	CONNECTED = "Connecté",
	ERROR = "Erreur",
	SEND = "Envoyé",
	RECV = "Reçu",
}

type WriteOutputFn = (_: State, ...args: unknown[]) => void;

function useWebSocketStore(write_output_fn: WriteOutputFn) {
	let ws = ref<WebSocket>();

	// ------ //
	// Export //
	// ------ //

	let state = reactive({ connection: 0 });

	function connect(url: string | URL) {
		ws.value = new WebSocket(url);

		let lws = ws.value;
		lws.binaryType = "arraybuffer";
		lws.addEventListener("open", handle_open_connection(write_output_fn));
		lws.addEventListener("close", handle_close_connection(write_output_fn));
		lws.addEventListener("message", handle_message(write_output_fn));
		lws.addEventListener("error", handle_error(write_output_fn));
		state.connection = lws.readyState;
	}

	function close() {
		let lws = ws.value;
		if (!lws) {
			return;
		}
		lws.close(1000, "Client Quit.");
		state.connection = lws.readyState;
		ws.value = undefined;
	}

	function write(data: string | ArrayBuffer) {
		let lws = ws.value;
		if (!lws) {
			return;
		}

		write_output_fn(State.SEND, data);
		state.connection = lws.readyState;

		let encoder = new TextEncoder();
		let bytes = encoder.encode(`${data}\r\n`);

		lws.send(bytes);
	}

	function on<K extends keyof WebSocketEventMap>(
		event: K,
		listener: (
			this: {
				close: typeof close;
				socket: WebSocket;
				write: typeof write;
			},
			socket: {
				close: typeof close;
				socket: WebSocket;
				write: typeof write;
			},
		) => any,
	) {
		let lws = ws.value;
		if (!lws) {
			return;
		}
		let cb = listener.bind({ close, socket: lws, write });
		lws.addEventListener(event, function () {
			cb({ close, socket: this, write });
			state.connection = this.readyState;
		});
	}

	return { state, close, connect, on, write };
}

function handle_open_connection(write_output_fn: WriteOutputFn) {
	return () => write_output_fn(State.CONNECTED);
}

function handle_close_connection(write_output_fn: WriteOutputFn) {
	return () => write_output_fn(State.CLOSED);
}

function handle_error(write_output_fn: WriteOutputFn) {
	return () => write_output_fn(State.ERROR);
}

function handle_message(write_output_fn: WriteOutputFn) {
	return (evt: MessageEvent) => {
		let raw = evt.data;
		if (raw instanceof Blob) {
			let file_reader = new FileReader();
			file_reader.addEventListener(
				"loadend",
				handle_binary_input(write_output_fn),
			);
			file_reader.readAsText(raw);
		} else {
			process(write_output_fn, raw);
		}
	};
}

function handle_binary_input(write_output_fn: WriteOutputFn) {
	return (evt: ProgressEvent<FileReader>) => {
		let file_reader = evt.target;
		let raw = file_reader?.result! as ArrayBuffer;
		process(write_output_fn, raw);
	};
}

function process(
	write_output_fn: WriteOutputFn,
	raw: NonNullable<ArrayBuffer>,
) {
	let decoder = new TextDecoder();
	let s = decoder.decode(raw);
	write_output_fn(State.RECV, s);
}

// ------ //
// Export //
// ------ //

export { useWebSocketStore, State as State };
