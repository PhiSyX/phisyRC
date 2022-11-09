<script lang="ts">
export default {
	name: "Sidebar",
	components: { Button },
};
</script>

<script lang="ts" setup>
import IconArrowLeft from "~vue/assets/icons/IconArrowLeft.vue";
import IconArrowRight from "~vue/assets/icons/IconArrowRight.vue";

import Button from "../components/Button.vue";
import SidebarList from "./SidebarList.vue";

import { computed } from "vue";

import type { Server } from "./server";

type Props = {
	toggle: boolean;
};

const props = defineProps<Props>();

let emit = defineEmits(["update:toggle"]);

let toggle$ = computed({
	get() {
		return props.toggle;
	},
	set($1: boolean) {
		emit("update:toggle", $1);
	},
});

let servers: Server[] = [];
</script>

<template>
	<nav
		role="navigation"
		class="sidebar [ flex! h:full ]"
		:class="{
			'is-opened': toggle,
			'is-collapsed': !toggle,
		}"
	>
		<ul
			class="app:bg=secondary network network<server> [ flex:full ][ scroll:y scroll:hidden list:reset ]"
		>
			<SidebarList
				v-for="server in servers"
				v-bind="server"
				:key="server.name"
			/>
		</ul>

		<footer class="sidebar__actions [ h=6 p=2 ]">
			<Button
				type="button"
				class="[ h:full mr=3 ]"
				v-model:toggle="toggle$"
			>
				<IconArrowLeft v-if="toggle" />
				<IconArrowRight v-else />
			</Button>
		</footer>
	</nav>
</template>
