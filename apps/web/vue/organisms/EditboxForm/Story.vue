<script lang="ts" setup>
import { reactive } from "vue";
import Editbox from "./EditboxForm.vue";

let room1 = reactive({
	input: "",
	input_history: [],
	input_foreground: 1,
	input_background: null,
});

let room2 = reactive({
	input: "",
	input_history: [],
	input_foreground: 4,
	input_background: 3,
});

let room3 = reactive({
	input: "",
	input_history: ["hello", "world"],
	input_foreground: 7,
	input_background: 4,
});
</script>

<template>
	<section class="[ flex! gap=2 p=1 ]">
		<div class="editbox-story [ w:full ]">
			<Editbox
				v-model="room1.input"
				v-model:history="room1.input_history"
				v-model:foreground="room1.input_foreground"
				v-model:background="room1.input_background"
			/>
		</div>

		<div class="editbox-story [ w:full ]">
			<Editbox
				v-model="room2.input"
				v-model:history="room2.input_history"
				v-model:foreground="room2.input_foreground"
				v-model:background="room2.input_background"
			/>
		</div>

		<div class="editbox-story [ w:full ]">
			<Editbox
				v-model="room3.input"
				v-model:history="room3.input_history"
				v-model:foreground="room3.input_foreground"
				v-model:background="room3.input_background"
			/>
		</div>
	</section>
</template>

<style scoped lang="scss">
@import "design/functions";
@import "design/mixins";

.editbox-story {
	@include --theme using ($name) {
		@if $name == light {
			background: var(--color-white);
		} @else if $name == dark {
			background: #31363a;
		}
	}
}
</style>
