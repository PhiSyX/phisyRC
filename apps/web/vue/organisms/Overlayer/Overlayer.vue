<script lang="ts">
export default {
	name: "Overlayer",
};
</script>

<script setup lang="ts">
import type { Layer } from "~vue/stores/overlayer";
import type { StyleValue } from "vue";

import OverlayerTeleport from "./Teleport.vue";

import { onBeforeMount, onBeforeUnmount } from "vue";

import vTrap from "~vue/directives/trap_focus";
import { use_overlayer_store } from "~vue/stores/overlayer";

let use_store = use_overlayer_store();
let store = use_store();

/// Événement de destruction d'un layer
///
/// Quand un ID est spécifié, seul le layer qui a cet ID est détruit.
function destroy(_: Event, id?: Layer["id"]) {
	if (id) {
		store.destroy_layer(id);
	} else {
		store.destroy_layers();
	}
}

/// Événement de mise à jour des layers
function resize() {
	store.update_layers();
}

// ----- //
// Hooks //
// ----- //

onBeforeMount(() => {
	window.addEventListener("resize", resize, { passive: true });
});

onBeforeUnmount(() => {
	window.removeEventListener("resize", resize);
});
</script>

<template>
	<transition name="fade">
		<div v-if="!store.has_layers" id="overlayer" v-trap:focus="{}">
			<div class="overlay [ pos-a:full ]" @click="destroy"></div>

			<div
				class="layer [ border:radius=1 ]"
				v-for="[id, layer] of store.layers"
				@keydown.esc="destroy($event, id)"
				:key="`${id}_layer`"
				:id="`${id}_layer`"
				:style="layer.style as StyleValue"
			></div>

			<OverlayerTeleport
				v-for="[id, layer] of store.layers"
				:key="`${id}_teleport`"
				:id="id"
				:layer="layer"
			/>
		</div>
	</transition>
</template>

<style lang="scss">
@import "design/app/organisms/overlayer";
</style>
