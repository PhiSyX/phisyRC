<script lang="ts" setup>
import { reactive } from "vue";
import Editbox from "./Editbox.vue";

let room1 = reactive({
	input: "",
});

let room2 = reactive({
	input: "",
});

let room3 = reactive({
	input: "",
});
</script>

<template>
	<section class="[ flex! gap=2 p=1 ]">
		<div id="editbox1" class="[ w:full ]">
			<Editbox v-model="room1.input" />
		</div>

		<div id="editbox2" class="[ w:full ]">
			<Editbox v-model="room2.input" />
		</div>

		<div id="editbox3" class="[ w:full ]">
			<Editbox v-model="room3.input" />
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
			background: var(--color-grey900);
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
