<script lang="ts">
export default {
	name: "Input",
	inheritAttrs: false,
};
</script>

<script lang="ts" setup>
import type { Props as InputProps } from "~/atoms/Input/props";
import { HTMLAttributes } from "vue";
import { use_model } from "~vue/hooks/use_models";

type Props = {
	name: InputProps<HTMLAttributes["class"]>["name"];
	datalist?: InputProps<HTMLAttributes["class"]>["datalist"];

	lclass?: InputProps<HTMLAttributes["class"]>["lclass"];
	rclass?: InputProps<HTMLAttributes["class"]>["rclass"];
	iclass?: InputProps<HTMLAttributes["class"]>["iclass"];
	dclass?: InputProps<HTMLAttributes["class"]>["dclass"];
	diclass?: InputProps<HTMLAttributes["class"]>["diclass"];
	diclick?: InputProps<HTMLAttributes["class"]>["diclick"];

	// NOTE(phisyx): v-model
	modelValue?: string | string[];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:modelValue"]);

let model$ = use_model(props)(emit);
</script>

<template>
	<div class="app@input" :class="rclass">
		<slot name="label">
			<label :class="lclass" :for="name" v-if="$slots.default">
				<slot />
			</label>
		</slot>

		<ol
			class="[ list:reset flex flex:wrap gap=1 ]"
			:class="dclass"
			v-if="datalist && datalist.length > 0"
		>
			<li
				class="[ align-s:start ]"
				:class="diclass?.(item, i)"
				@click="diclick?.($event, item, i)"
				v-for="(item, i) in datalist"
			>
				{{ item }}
			</li>

			<li class="[ flex:full ]">
				<input
					:id="name"
					:name="name"
					class="[ input:reset ]"
					v-bind="$attrs"
					v-model="model$"
				/>
			</li>
		</ol>
		<input
			v-else
			:id="name"
			:name="name"
			class="[ input:reset ]"
			v-bind="$attrs"
			v-model="model$"
		/>

		<span :class="iclass" v-if="$slots.icon">
			<slot name="icon" />
		</span>
	</div>
</template>
