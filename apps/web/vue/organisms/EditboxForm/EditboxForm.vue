<script lang="ts">
export default {
	name: "Editbox",
};
</script>

<script lang="ts" setup>
import IconSendMessage from "~vue/atoms/Icons/IconSendMessage.vue";

import Button from "~vue/atoms/Button/Button.vue";

import { use_model } from "~vue/hooks/use_models";

import EditboxInput from "./EditboxInput.vue";

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
</script>

<template>
	<form
		autocapitalize="off"
		method="post"
		class="editbox [ flex gap=1 p=1 border:radius=1 ]"
		:style="{
			'--user-fg-color': `var(--color-irc${foreground})`,
			'--user-fg-color_hsl': `var(--color-irc${foreground}_hsl)`,
			'--user-bg-color': `var(--color-irc${background})`,
			'--user-bg-color_hsl': `var(--color-irc${background}_hsl)`,
		}"
		@submit.prevent
	>
		<EditboxInput
			v-model="input$"
			v-model:history="history$"
			v-model:foreground="foreground$"
			v-model:background="background$"
		/>

		<Button class="[ border:radius=full [ p=1 ] ]" type="submit">
			<IconSendMessage />
		</Button>
	</form>
</template>

<style lang="scss">
@import "design/app/organisms/editbox-form";
</style>
