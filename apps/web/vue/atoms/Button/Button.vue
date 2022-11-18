<script lang="ts">
export default {
	name: "Button",
};
</script>

<script lang="ts" setup>
import { nextTick } from "vue";

type Props = {
	toggle?: boolean;
};

const props = defineProps<Props>();

const emit = defineEmits(["update:toggle"]);

function toggle_click_handler(evt: MouseEvent) {
	emit("update:toggle", !props.toggle);

	if (props.toggle === false) {
		let target = evt.currentTarget as HTMLButtonElement;
		nextTick(() => target.blur());
	}
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
