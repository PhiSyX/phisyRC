<script setup lang="ts">
import { computed } from "vue";
import { use_model } from "~vue/hooks/use_models";

type Props = {
	// v-model
	modelValue: number | null;
};

const props = defineProps<Props>();
const emit = defineEmits(["update:modelValue"]);

const color$ = use_model(props)(emit);

const generate_id = computed(() => (id: number, idx: number) => {
	return `text-color_${id + "" + idx}`;
});

const generate_color = computed(() => (idx: number) => {
	return {
		"--color": `var(--color-irc${idx})`,
	};
});
</script>

<template>
	<div class="dialog@text-format:color [ gap=1 align-j:space-evenly ]">
		<label
			v-for="idx of 16"
			class="[ size=2 border:radius=1 ]"
			title="Maintenez CTRL pour enlever la couleur"
			:key="idx"
			:for="generate_id($.uid, idx - 1)"
			:style="generate_color(idx - 1)"
			:class="{
				'opacity=1': modelValue === idx - 1,
				'opacity=1(:hover) opacity=.5': modelValue !== idx - 1,
			}"
			@click.ctrl="color$ = null"
		>
			<input
				v-model="color$"
				:id="generate_id($.uid, idx - 1)"
				:value="idx - 1"
				type="radio"
				name="color"
				class="[ sr-only ]"
			/>
		</label>
	</div>
</template>
