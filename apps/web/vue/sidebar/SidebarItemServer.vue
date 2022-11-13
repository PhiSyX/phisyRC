<script lang="ts">
export default {
	name: "SidebarItemServer",
};
</script>

<script lang="ts" setup>
import { computed } from "vue";

import IconArrowDown from "~vue/assets/icons/IconArrowDown.vue";
import IconArrowRight from "~vue/assets/icons/IconArrowRight.vue";
import IconServerConnect from "~vue/assets/icons/IconServerConnect.vue";

import Button from "~vue/components/Button.vue";

type Props = {
	name: string;
	folded: boolean;
};

const props = defineProps<Props>();

const emit = defineEmits(["update:folded"]);

let folded$ = computed({
	get() {
		return props.folded;
	},
	set($1: boolean) {
		emit("update:folded", $1);
	},
});
</script>

<template>
	<li class="app:bg=primary" data-type="server">
		<div>
			<IconServerConnect />
		</div>

		<div class="[ align-t:center ]">{{ name }}</div>

		<div class="network@server__actions">
			<Button v-model:toggle="folded$">
				<IconArrowRight v-if="folded" />
				<IconArrowDown v-else />
			</Button>
		</div>
	</li>
</template>
