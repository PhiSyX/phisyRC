<script lang="ts">
export default {
	name: "Sidebar",
};
</script>

<script lang="ts" setup>
import IconArrowLeft from "~vue/assets/icons/IconArrowLeft.vue";
import IconArrowRight from "~vue/assets/icons/IconArrowRight.vue";
import IconMessages from "~vue/assets/icons/IconMessages.vue";
import IconSettings from "~vue/assets/icons/IconSettings.vue";

import { computed } from "vue";

import type { Server } from "~/server";

import Button from "~vue/components/Button.vue";
import SidebarList from "./SidebarList.vue";

type Props = {
	toggle: boolean;
	servers: Server[];
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

		<footer class="sidebar__actions [ flex h=6 px=1 ]">
			<Button
				type="button"
				class="[ h:full mr=3 ]"
				v-model:toggle="toggle$"
			>
				<IconArrowLeft v-if="toggle" />
				<IconArrowRight v-else />
			</Button>

			<div class="[ flex:full ][ flex align-i:center gap=2 pr=2 ]">
				<Button
					class="[ h:full ]"
					title="Vos messages privés en attente"
				>
					<IconMessages />
				</Button>

				<Button
					class="[ h:full ]"
					title="Configuration de la barre latérale"
				>
					<IconSettings />
				</Button>
			</div>
		</footer>
	</nav>
</template>
