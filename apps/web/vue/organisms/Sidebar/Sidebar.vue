<script lang="ts">
export default {
	name: "Sidebar",
};
</script>

<script lang="ts" setup>
import type { Server } from "~/server";

import { computed } from "vue";

import IconArrowLeft from "~vue/atoms/Icons/IconArrowLeft.vue";
import IconArrowRight from "~vue/atoms/Icons/IconArrowRight.vue";
import IconMessages from "~vue/atoms/Icons/IconMessages.vue";
import IconSettings from "~vue/atoms/Icons/IconSettings.vue";

import Button from "~vue/atoms/Button/Button.vue";
import SidebarList from "~vue/organisms/Sidebar/SidebarList.vue";

type Props = {
	toggle: boolean;
	servers: Server[];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:toggle"]);

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
		class="sidebar sidebar<svg> network [ flex! h:full scroll:y scroll:hidden ]"
		:class="{
			'is-opened': toggle,
			'is-collapsed': !toggle,
		}"
	>
		<div class="[ flex:full scroll:y scroll:hidden ]">
			<SidebarList
				v-for="server in servers"
				v-bind="server"
				:key="server.name"
				v-model:folded="server.is_folded"
			/>
		</div>

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
