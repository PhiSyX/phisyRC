<script setup lang="ts">
import type { StyleValue } from "vue";
import type { Layer } from "~vue/stores/overlayer";

import { computed, ref, watch, onBeforeMount, onBeforeUnmount } from "vue";
import { to_px } from "@phisyrc/css";

import { use_overlayer_store } from "~vue/stores/overlayer";

type Props = {
	id: Layer["id"];
	layer: Layer;
};

const use_store = use_overlayer_store();
const store = use_store();
const props = defineProps<Props>();

let $el = ref<HTMLDivElement>();

const deltaX$ = computed(() => props.layer.mouse_position["left"]);
const deltaY$ = computed(() => props.layer.mouse_position["top"]);

function update_position(dom_el: HTMLDivElement) {
	if (props.layer.centered) {
		return;
	}

	const { innerWidth, innerHeight } = window;

	let { clientX: deltaX, clientY: deltaY } = props.layer.event;

	let doc_position_element = document.body.getBoundingClientRect();
	let dom_position_teleport_element =
		dom_el?.firstElementChild?.getBoundingClientRect()!;

	if (
		deltaX >=
		doc_position_element.width -
			dom_position_teleport_element.width -
			(innerWidth - doc_position_element.width)
	) {
		deltaX =
			doc_position_element.width - dom_position_teleport_element.width;
	}

	if (
		deltaY >=
		doc_position_element.height -
			dom_position_teleport_element.height -
			(innerHeight - doc_position_element.height)
	) {
		deltaY =
			doc_position_element.height - dom_position_teleport_element.height;
	}

	let mouse_position: Layer["mouse_position"] = {
		left: to_px(deltaX),
		top: to_px(deltaY),
	};

	store.update_layer_mut(props.id, { mouse_position });
}

function resize() {
	update_position($el.value!);
}

// ----- //
// Hooks //
// ----- //

watch($el, (dom_el) => update_position(dom_el!));

onBeforeMount(() => {
	window.addEventListener("resize", resize, { passive: true });
});

onBeforeUnmount(() => {
	window.removeEventListener("resize", resize);
});
</script>

<template>
	<div
		ref="$el"
		:id="`${id}_teleport`"
		tabindex="0"
		class="teleport [ pos-a:full flex! ]"
		:class="{
			'[ align-i:center ]': layer.centered,
		}"
		:style="{
			left: deltaX$,
			top: deltaY$,
		} as StyleValue"
	></div>
</template>
