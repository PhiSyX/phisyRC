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
import { handle_keydown } from "~/organisms/EditboxForm/handler";

type Props = {
	// v-model:history
	history: string[];
	// v-model
	modelValue: string;
};

const props = defineProps<Props>();
const emit = defineEmits(["update:modelValue", "update:history"]);

const input$ = use_model(props)(emit);
const history$ = use_model(props, "history")(emit);

const id = uuid();

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
</template>

<style lang="scss">
@import "design/app/organisms/editbox-input";
</style>
