<script lang="ts">
export default {
	name: "Editbox",
};
</script>

<script lang="ts" setup>
import IconAttachFile from "~vue/atoms/Icons/IconAttachFile.vue";
import IconSendMessage from "~vue/atoms/Icons/IconSendMessage.vue";
import IconVoiceRecording from "~vue/atoms/Icons/IconVoiceRecording.vue";

import Input from "~vue/atoms/Input/Input.vue";
import Button from "~vue/atoms/Button/Button.vue";

import { ref } from "vue";
import { uuid } from "@phisyrc/std";
import { use_model } from "~vue/hooks/use_models";

type Props = {
	// v-model
	modelValue: string;
};

const props = defineProps<Props>();
const emit = defineEmits(["update:modelValue"]);

const input$ = use_model(props)(emit);

const id = uuid();

let voice_recording = ref(false);
let attach_file = ref(false);
let text_format = ref(false);
</script>

<template>
	<form method="post" class="editbox [ p=1 border:radius=1 ]" @submit.prevent>
		<Input
			rclass="[ flex align-i:center gap=1 ]"
			iclass="[ flex gap=1 ]"
			autocomplete="off"
			class="editbox@input:user-color [ flex:full ][ box-fd:shadow f-family=roboto ]"
			placeholder="Envoyer un message"
			:name="`message-${id}`"
			v-model="input$"
		>
			<template #label>
				<Button v-model:toggle="voice_recording">
					<IconVoiceRecording />
				</Button>

				<Button v-model:toggle="attach_file">
					<IconAttachFile />
				</Button>
			</template>

			<template #icon>
				<Button
					v-model:toggle="text_format"
					style="font-size: 24px; color: var(--user-fg-color)"
				>
					<span class="[ svg ] [ text:underline ]">A</span>
				</Button>

				<Button class="[ border:radius=full [ p=1 ] ]" type="submit">
					<IconSendMessage />
				</Button>
			</template>
		</Input>
	</form>
</template>

<style lang="scss">
@import "design/app/molecules/editbox";
</style>
