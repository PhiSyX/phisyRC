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
	// v-model:history
	history: string[];
	// v-model
	modelValue: string;
};

const props = defineProps<Props>();
const emit = defineEmits(["update:modelValue", "update:history"]);

const input$ = use_model(props)(emit);
const history$ = use_model(props, "history")(emit);
</script>

<template>
	<form
		autocapitalize="off"
		method="post"
		class="editbox [ flex gap=1 p=1 border:radius=1 ]"
		@submit.prevent
	>
		<EditboxInput v-model="input$" v-model:history="history$" />

		<Button class="[ border:radius=full [ p=1 ] ]" type="submit">
			<IconSendMessage />
		</Button>
	</form>
</template>

<style lang="scss">
@import "design/app/organisms/editbox-form";
</style>
