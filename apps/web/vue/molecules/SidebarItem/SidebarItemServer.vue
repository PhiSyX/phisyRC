<script lang="ts">
export default {
	name: "SidebarItemServer",
	inheritAttrs: false,
};
</script>

<script lang="ts" setup>
import IconArrowDown from "~vue/atoms/Icons/IconArrowDown.vue";
import IconArrowRight from "~vue/atoms/Icons/IconArrowRight.vue";
import IconServerConnect from "~vue/atoms/Icons/IconServerConnect.vue";

import Button from "~vue/atoms/Button/Button.vue";

import type { ServerProps as SidebarItemServerProps } from "~/molecules/SidebarItem/props";
import { use_model } from "~vue/hooks/use_models";

type Props = {
	name: SidebarItemServerProps["name"];

	// NOTE(phisyx): v-model:folded
	folded: SidebarItemServerProps["is_folded"];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:folded"]);

let folded$ = use_model(props, "folded")(emit);
</script>

<template>
	<li class="app:bg=primary sidebar@item" data-type="server">
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
