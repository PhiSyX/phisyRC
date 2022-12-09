<script lang="ts" setup>
import { reactive } from "vue";
import Editbox from "./EditboxForm.vue";

let room1 = reactive({
	input: "",
	input_history: [],
});

let room2 = reactive({
	input: "",
	input_history: [],
});

let room3 = reactive({
	input: "",
	input_history: ["hello", "world"],
});
</script>

<template>
	<section class="[ flex! gap=2 p=1 ]">
		<div id="editbox1" class="[ w:full ]">
			<Editbox
				v-model="room1.input"
				v-model:history="room1.input_history"
			/>
		</div>

		<div id="editbox2" class="[ w:full ]">
			<Editbox
				v-model="room2.input"
				v-model:history="room2.input_history"
			/>
		</div>

		<div id="editbox3" class="[ w:full ]">
			<Editbox
				v-model="room3.input"
				v-model:history="room3.input_history"
			/>
		</div>
	</section>
</template>

<style scoped lang="scss">
@import "design/functions";
@import "design/mixins";

#editbox1,
#editbox2,
#editbox3 {
	@include --theme using ($name) {
		@if $name == light {
			background: var(--color-white);
		} @else if $name == dark {
			background: #31363a;
		}
	}
}

#editbox2 {
	--user-fg-color: var(--color-irc4);
	--user-fg-color_hsl: var(--color-irc4_hsl);
	--user-bg-color: var(--color-irc3);
}
#editbox3 {
	--user-fg-color: var(--color-irc7);
	--user-fg-color_hsl: var(--color-irc7_hsl);
	--user-bg-color: var(--color-irc4);
}
</style>
