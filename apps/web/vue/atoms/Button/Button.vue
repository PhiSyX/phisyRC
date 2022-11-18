<script lang="ts">
export default {
	name: "Button",
};
</script>

<script lang="ts" setup>
import type { Props as ButtonProps } from "~/atoms/Button/props";

import { toggle_click } from "~/atoms/Button/handler";

type Props = {
	// v-model:toggle
	toggle?: ButtonProps["toggle"];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:toggle"]);

function toggle_click_handler(evt: MouseEvent) {
	toggle_click(evt, !props.toggle, ($1) => emit("update:toggle", $1));
}
</script>

<template>
	<button
		type="button"
		class="app@btn [ btn:reset ]"
		@click="toggle_click_handler"
	>
		<slot />
	</button>
</template>

<style lang="scss">
@import "design/app/atoms/button";
</style>
