<script lang="ts">
export default {
	name: "Input",
	inheritAttrs: false,
};
</script>

<script lang="ts" setup>
import type { Props as InputProps } from "~/atoms/Input/props";
import { computed, HTMLAttributes } from "vue";

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

let model = computed({
	get() {
		return props.modelValue;
	},
	set($1) {
		emit("update:modelValue", $1);
	},
});
</script>

<template>
	<div class="app@input" :class="rclass">
		<label :class="lclass" :for="name" v-if="$slots.default">
			<slot />
		</label>

		<ol
			class="[ list:reset flex flex:wrap gap=1 ]"
			:class="dclass"
			v-if="datalist && datalist.length > 0"
		>
			<li
				class="[ align-s:start ]"
				:class="diclass?.(i)"
				@click="diclick?.($event, i)"
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
					v-model="model"
				/>
			</li>
		</ol>
		<input
			v-else
			:id="name"
			:name="name"
			class="[ input:reset ]"
			v-bind="$attrs"
			v-model="model"
		/>

		<span :class="iclass" v-if="$slots.icon">
			<slot name="icon" />
		</span>
	</div>
</template>
