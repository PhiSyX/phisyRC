<script lang="ts">
import { defineAsyncComponent } from "vue";

const ColorsStory = defineAsyncComponent(
	() => import("~vue/stories/Colors.vue")
);
const TypographyStory = defineAsyncComponent(
	() => import("~vue/stories/Typography.vue")
);
const SidebarStory = defineAsyncComponent(
	() => import("~vue/stories/Sidebar.vue")
);

export default {
	name: "UI",
	components: {
		ColorsStory,
		TypographyStory,
		SidebarStory,
	},
};
</script>

<script lang="ts" setup>
import IconAdd from "~vue/assets/icons/IconAdd.vue";
import IconArrowDown from "~vue/assets/icons/IconArrowDown.vue";
import IconArrowLeft from "~vue/assets/icons/IconArrowLeft.vue";
import IconArrowRight from "~vue/assets/icons/IconArrowRight.vue";
import IconArrowUp from "~vue/assets/icons/IconArrowUp.vue";
import IconChannel from "~vue/assets/icons/IconChannel.vue";
import IconColor from "~vue/assets/icons/IconColor.vue";
import IconCross from "~vue/assets/icons/IconCross.vue";
import IconMessage from "~vue/assets/icons/IconMessage.vue";
import IconMessageEmpty from "~vue/assets/icons/IconMessageEmpty.vue";
import IconPassword from "~vue/assets/icons/IconPassword.vue";
import IconServerConnect from "~vue/assets/icons/IconServerConnect.vue";
import IconTrashDelete from "~vue/assets/icons/IconTrashDelete.vue";
import IconUser from "~vue/assets/icons/IconUser.vue";
import IconValidated from "~vue/assets/icons/IconValidated.vue";
import IconVisualPassword from "~vue/assets/icons/IconVisualPassword.vue";

import { capitalize, computed, onMounted } from "vue";
import { useRoute } from "vue-router";

onMounted(() => {
	document.documentElement.dataset["js"] = "on";
	document.title = "Design System | phisyRC";
});

const route = useRoute();

const props = defineProps(["dyncomponent"]);

const load_component = computed(() => {
	return capitalize(props.dyncomponent) + "Story";
});

type List = {
	icon?: unknown;
	text: string;
	link?: string;
};

let general = [
	{
		icon: IconColor,
		text: "Les couleurs",
		link: "/ui/colors",
	},
	{
		text: "Typographie",
		link: "/ui/typography",
	},
];

let components: List[] = [];

let icons: List[] = [
	{ icon: IconAdd, text: "Add" },
	{ icon: IconArrowDown, text: "ArrowDown" },
	{ icon: IconArrowLeft, text: "ArrowLeft" },
	{ icon: IconArrowRight, text: "ArrowRight" },
	{ icon: IconArrowUp, text: "ArrowUp" },
	{ icon: IconChannel, text: "Channel" },
	{ icon: IconCross, text: "Cross" },
	{ icon: IconMessage, text: "Message" },
	{ icon: IconMessageEmpty, text: "MessageEmpty" },
	{ icon: IconPassword, text: "Password" },
	{ icon: IconServerConnect, text: "ServerConnect" },
	{ icon: IconTrashDelete, text: "TrashDelete" },
	{ icon: IconUser, text: "User" },
	{ icon: IconValidated, text: "Validated" },
	{ icon: IconVisualPassword, text: "VisualPassword" },
];

let application: List[] = [
	{
		text: "Barre latérale",
		link: "/ui/sidebar",
	},
];
</script>

<template>
	<div id="ui-page" class="[ flex size:full gap=1 p=1 ]">
		<nav class="[ flex! p=1 border:radius=1 f-family=roboto ]">
			<p class="[ align-t:center text:bold ]">
				Design System pour phisyRC
			</p>

			<details open>
				<summary class="[ pl=1 pb=1 ]">Général</summary>

				<ul class="[ flex! gap=1 list:reset ]">
					<li
						class="[ pos-r flex align-i:center gap=1 px=1 border:radius=2 ]"
						v-for="item in general"
						:class="{
							active: item.link == route.fullPath,
						}"
					>
						<component :is="item.icon" width="20" height="20" />

						<span class="[ flex:full ]">{{ item.text }}</span>

						<RouterLink
							v-if="item.link"
							:to="item.link"
							class="pos-a:full"
						></RouterLink>
					</li>
				</ul>
			</details>

			<details open>
				<summary class="[ pl=1 pb=1 ]">Composants globaux</summary>

				<ul class="[ flex! gap=1 list:reset ]">
					<li
						class="[ pos-r flex align-i:center gap=1 px=1 border:radius=2 ]"
						v-for="item in components"
						:class="{
							active: item.link == route.fullPath,
						}"
					>
						<component :is="item.icon" width="20" height="20" />

						<span class="[ flex:full ]">{{ item.text }}</span>

						<RouterLink
							v-if="item.link"
							:to="item.link"
							class="pos-a:full"
						></RouterLink>
					</li>
				</ul>
			</details>

			<details open>
				<summary class="[ pl=1 pb=1 ]">Application</summary>

				<ul class="[ flex! gap=1 list:reset scroll:y ]">
					<li
						class="[ pos-r flex align-i:center gap=1 px=1 border:radius=2 ]"
						v-for="item in application"
					>
						<span class="[ flex:full ]">{{ item.text }}</span>

						<RouterLink
							v-if="item.link"
							:to="item.link"
							class="pos-a:full"
						></RouterLink>
					</li>
				</ul>
			</details>

			<details>
				<summary class="[ pl=1 pb=1 ]">Les icônes</summary>

				<ul class="[ flex! gap=1 list:reset scroll:y ]">
					<li
						class="[ pos-r flex align-i:center gap=1 px=1 border:radius=2 ]"
						v-for="item in icons"
					>
						<component :is="item.icon" width="20" height="20" />

						<span class="[ flex:full ]">{{ item.text }}</span>

						<RouterLink
							v-if="item.link"
							:to="item.link"
							class="pos-a:full"
						></RouterLink>
					</li>
				</ul>
			</details>
		</nav>

		<main
			class="[ flex:full flex! gap=1 p=1 border:radius=2 scroll:y ]"
			:class="{
				'align-i:center': !dyncomponent,
			}"
		>
			<template v-if="dyncomponent">
				<h1 class="[ heading=5 m=0 p=1 border:radius=2 ]">
					{{ load_component }}
				</h1>

				<section class="[ flex:full ]">
					<component :is="load_component">
						Sélectionner un composant
					</component>
				</section>
			</template>
			<template v-else> <p>Sélectionner un composant</p> </template>
		</main>
	</div>
</template>

<style lang="scss" scoped>
@import "design/functions";
@import "design/mixins";

main {
	@include --theme using($name) {
		@if $name == dark {
			border: 1px solid var(--color-grey800);
		} @else if $name == light {
			border: 1px solid var(--color-grey400);
		}
	}
}

main > h1 {
	color: var(--color-grey500);
}

nav {
	width: space(320);
	font-size: 14px;

	@include --theme using($name) {
		@if $name == dark {
			background-color: var(--color-white);
			color: var(--color-grey900);
		} @else if $name == light {
			background-color: var(--color-grey900);
			color: var(--color-snow);
		}
	}
}

details > summary {
	margin-left: -8px;
	list-style: none;
	color: var(--color-grey);
	font-size: 13px;
}

ul {
	max-height: space(250);
}

li {
	height: space(32);

	&:hover {
		@include --theme using($name) {
			@if $name == dark {
				background-color: var(--color-grey200);
			} @else if $name == light {
				background-color: var(--color-grey800);
			}
		}
	}
}

li.active {
	color: var(--color-snow);

	@include --theme using($name) {
		@if $name == dark {
			background: var(--color-grey900);
		} @else if $name == light {
			background-color: var(--color-grey800);
		}
	}
}
</style>
