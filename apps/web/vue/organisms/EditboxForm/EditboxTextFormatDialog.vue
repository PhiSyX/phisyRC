<script lang="ts" setup>
import { ref } from "vue";
import { use_model } from "~vue/hooks/use_models";

import EditboxTextColor from "./EditboxTextColor.vue";

type Props = {
	name: string;

	// v-model:foreground
	foreground: number;
	// v-model:background
	background: number | null;
};

const props = defineProps<Props>();
const emit = defineEmits(["update:foreground", "update:background"]);

const foreground$ = use_model(props, "foreground")(emit);
const background$ = use_model(props, "background")(emit);

let page = ref<"foreground" | "background">("foreground");
</script>

<template>
	<dialog open class="dialog@text-format [ flex! gap=1 p=2 border:radius=2 ]">
		<header class="[ flex gap=1 ]">
			<a
				href="#"
				:class="{ 'is-active': page === 'foreground' }"
				@click.prevent="page = 'foreground'"
			>
				Premier plan
			</a>

			<a
				href="#"
				:class="{ 'is-active': page === 'background' }"
				@click.prevent="page = 'background'"
			>
				Arrière plan
			</a>
		</header>

		<form action="#" method="POST">
			<fieldset class="[ mb=2 border:radius=2 ]">
				<legend class="[ mx=1 ]">Prévisualisation</legend>

				<div
					class="[ p=1 border:radius=2 ]"
					:style="{
						color: `var(--color-irc${foreground})`,
						background: `hsl(var(--color-irc${background}_hsl), .25)`,
					}"
				>
					Exemple de message
				</div>
			</fieldset>

			<EditboxTextColor
				v-if="page === 'foreground'"
				v-model="foreground$"
			/>
			<EditboxTextColor
				v-if="page === 'background'"
				v-model="background$"
			/>
		</form>
	</dialog>
</template>

<style lang="scss">
@import "design/app/organisms/editbox-dialog";
</style>
