<script lang="ts" setup>
import { reactive } from "vue";
import Topic from "./Topic.vue";

let room1 = reactive({
	topic: "",
	editable: false,
});

let room2 = reactive({
	topic: "",
	editable: true,
});

let room3 = reactive({
	topic: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Aperiam veritatis sunt, aut veniam impedit mollitia nam explicabo. Nihil, nobis doloremque sequi modi aut ex eligendi tenetur totam ullam, adipisci asperiores.",
	editable: true,
});
</script>

<template>
	<section class="[ flex! gap=2 p=1 ]">
		<div id="topic1" class="[ w:full ]">
			<Topic v-model:topic="room1.topic" :is_editable="room1.editable" />
		</div>
		<div id="topic2" class="[ w:full ]">
			<Topic v-model:topic="room2.topic" :is_editable="room2.editable" />
		</div>
		<div id="topic3" class="[ w:full ]">
			<Topic v-model:topic="room3.topic" :is_editable="room3.editable" />
		</div>
	</section>
</template>

<style scoped lang="scss">
@import "design/functions";
@import "design/mixins";

#topic1,
#topic2,
#topic3 {
	@include --theme using ($name) {
		@if $name == light {
			background: var(--color-white);
		} @else if $name == dark {
			background: var(--color-grey900);
		}
	}
}
</style>
