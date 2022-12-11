<script lang="ts" setup>
import IconAttachFile from "~vue/atoms/Icons/IconAttachFile.vue";
import IconVoiceRecording from "~vue/atoms/Icons/IconVoiceRecording.vue";

import Button from "~vue/atoms/Button/Button.vue";
import Input from "~vue/atoms/Input/Input.vue";

import EditboxAttachmentDialog from "./EditboxAttachmentDialog.vue";
import EditboxTextFormatDialog from "./EditboxTextFormatDialog.vue";

import { ref } from "vue";
import { uuid } from "@phisyrc/std";
import { use_model } from "~vue/hooks/use_models";
import { use_overlayer_store } from "~vue/stores/overlayer";
import { handle_keydown } from "~/organisms/EditboxForm/handler";

type Props = {
	// v-model:foreground
	foreground: number;
	// v-model:background
	background: number | null;
	// v-model:history
	history: string[];
	// v-model
	modelValue: string;
};

const props = defineProps<Props>();
const emit = defineEmits([
	"update:modelValue",
	"update:history",
	"update:foreground",
	"update:background",
]);

const input$ = use_model(props)(emit);
const history$ = use_model(props, "history")(emit);
const foreground$ = use_model(props, "foreground")(emit);
const background$ = use_model(props, "background")(emit);

const id = uuid();

let store_init = use_overlayer_store();
let store = store_init();

let current_input_history_index = ref(props.history.length);

let voice_recording = ref(false);
let attach_file = ref(false);
let text_format = ref(false);

function on_history_handler(evt: KeyboardEvent) {
	handle_keydown(
		evt,
		{
			data: input$.value,
			update(input) {
				input$.value = input;
			},
		},
		{
			data: history$.value,
			current: current_input_history_index.value,
			update(index) {
				current_input_history_index.value = index;
			},
		}
	);
}

function open_file_attachment_dialog(evt: MouseEvent) {
	store.create_layer({
		id: `editbox-file-attachment${id}`,
		event: evt,
		dom_element: evt.currentTarget as Element,
		before_destroy() {
			attach_file.value = false;
		},
	});
}

function open_text_format_dialog(evt: MouseEvent) {
	store.create_layer({
		id: `editbox-text-format${id}`,
		event: evt,
		dom_element: evt.currentTarget as Element,
		before_destroy() {
			text_format.value = false;
		},
	});
}
</script>

<template>
	<Input
		rclass="[ flex:full ][ flex align-i:center gap=1 p=1 border:radius=1 ]"
		iclass="[ flex gap=1 ]"
		class="editbox@input:user-color [ flex:full ][ box-fd:shadow f-family=roboto ]"
		autocomplete="off"
		placeholder="Envoyer un message"
		:name="`message-${id}`"
		v-model="input$"
		@keydown="on_history_handler"
	>
		<template #label>
			<Button v-model:toggle="voice_recording">
				<IconVoiceRecording />
			</Button>

			<Button
				v-model:toggle="attach_file"
				@click="open_file_attachment_dialog"
			>
				<IconAttachFile />
			</Button>
		</template>

		<template #icon>
			<Button
				v-model:toggle="text_format"
				style="font-size: 24px; color: var(--user-fg-color)"
				@click="open_text_format_dialog"
			>
				<span class="[ svg ] [ text:underline ]">A</span>
			</Button>
		</template>
	</Input>

	<Teleport
		v-if="store.layers.has(`editbox-file-attachment${id}`)"
		:to="`#editbox-file-attachment${id}_teleport`"
	>
		<EditboxAttachmentDialog :name="`editbox-file-attachment${id}`" />
	</Teleport>

	<Teleport
		v-if="store.layers.has(`editbox-text-format${id}`)"
		:to="`#editbox-text-format${id}_teleport`"
	>
		<EditboxTextFormatDialog
			:name="`editbox-text-format${id}`"
			v-model:foreground="foreground$"
			v-model:background="background$"
		/>
	</Teleport>
</template>

<style lang="scss">
@import "design/app/organisms/editbox-input";
</style>
