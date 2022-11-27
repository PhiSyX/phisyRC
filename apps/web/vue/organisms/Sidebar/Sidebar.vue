<script lang="ts">
export default {
	name: "Sidebar",
};
</script>

<script lang="ts" setup>
import IconArrowLeft from "~vue/atoms/Icons/IconArrowLeft.vue";
import IconArrowRight from "~vue/atoms/Icons/IconArrowRight.vue";
import IconMessages from "~vue/atoms/Icons/IconMessages.vue";
import IconSettings from "~vue/atoms/Icons/IconSettings.vue";

import Button from "~vue/atoms/Button/Button.vue";

import SidebarList from "~vue/molecules/SidebarList/SidebarList.vue";

import type { Props as SidebarProps } from "~/organisms/Sidebar/props";
import { use_model } from "~vue/hooks/use_models";

type Props = {
	servers: SidebarProps["servers"];

	// NOTE(phisyx): v-model:toggle
	toggle: SidebarProps["is_sidebar_opened"];
};

const props = defineProps<Props>();

const emit = defineEmits(["update:toggle"]);

let toggle$ = use_model(props, "toggle")(emit);
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
				class="network<server>"
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

<style lang="scss">
@import "design/app/organisms/sidebar";
</style>
