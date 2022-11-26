<script lang="ts">
export default {
	name: "Topic",
};
</script>

<script lang="ts" setup>
import IconSettingsVertical from "~vue/atoms/Icons/IconSettingsVertical.vue";
import IconNicklist from "~vue/atoms/Icons/IconNicklist.vue";
import IconCross from "~vue/atoms/Icons/IconCross.vue";

import Button from "~vue/atoms/Button/Button.vue";

import { computed, nextTick, ref } from "vue";

type Props = {
	topic: string;
	is_editable: boolean;
};

const props = defineProps<Props>();

const emit = defineEmits(["update:topic"]);

const topic$ = computed({
	get() {
		return props.topic;
	},
	set($1) {
		emit("update:topic", $1);
	},
});

let $input = ref<HTMLInputElement>();
let edit_mode = ref(false);

function enable_edit_mode() {
	if (!props.is_editable) {
		return;
	}

	edit_mode.value = true;
	nextTick(() => $input.value?.focus());
}

function disable_edit_mode(evt: Event) {
	evt.preventDefault();

	edit_mode.value = false;
}
</script>

<template>
	<div class="topic [ flex gap=1 p=1 w:full h=7 ]">
		<div class="[ flex:full ]" @dblclick="enable_edit_mode">
			<p
				v-if="!edit_mode"
				class="[ m=0 scroll:y ]"
				:class="{
					'is-editable': is_editable,
				}"
			>
				{{ topic }}
			</p>
			<form v-else action="" method="POST" @submit="disable_edit_mode">
				<input
					ref="$input"
					type="text"
					class="[ input:reset w:full ]"
					v-model="topic$"
					@blur="disable_edit_mode"
				/>
			</form>
		</div>

		<div class="[ flex gap=1 ]">
			<Button>
				<IconSettingsVertical />
			</Button>

			<Button>
				<IconNicklist />
			</Button>

			<Button>
				<IconCross />
			</Button>
		</div>
	</div>
</template>

<style scoped>
p {
	word-break: break-all;
}

p:empty::before {
	content: "Aucun sujet";
	font-size: small;
	font-style: italic;
	color: var(--color-grey);
}

.is-editable {
	cursor: pointer;
	align-self: center;
}
.is-editable:empty::before {
	content: "Double cliquer pour définir un sujet";
}
</style>
