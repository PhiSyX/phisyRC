<script lang="ts">
import { defineAsyncComponent } from "vue";

const StoryColors = defineAsyncComponent(
	() => import("~vue/stories/StoryColors.vue")
);
const StoryTypography = defineAsyncComponent(
	() => import("~vue/stories/StoryTypography.vue")
);
const StoryUtilities = defineAsyncComponent(
	() => import("~vue/stories/StoryUtilities.vue")
);
const StorySidebar = defineAsyncComponent(
	() => import("~vue/stories/StorySidebar.vue")
);

export default {
	name: "UI",
	components: {
		StoryColors,
		StoryTypography,
		StoryUtilities,
		StorySidebar,
	},
};
</script>

<script lang="ts" setup>
import IconAdd from "~vue/atoms/Icons/IconAdd.vue";
import IconArrowDown from "~vue/atoms/Icons/IconArrowDown.vue";
import IconArrowLeft from "~vue/atoms/Icons/IconArrowLeft.vue";
import IconArrowRight from "~vue/atoms/Icons/IconArrowRight.vue";
import IconArrowUp from "~vue/atoms/Icons/IconArrowUp.vue";
import IconChannel from "~vue/atoms/Icons/IconChannel.vue";
import IconColor from "~vue/atoms/Icons/IconColor.vue";
import IconCross from "~vue/atoms/Icons/IconCross.vue";
import IconMessage from "~vue/atoms/Icons/IconMessage.vue";
import IconMessageEmpty from "~vue/atoms/Icons/IconMessageEmpty.vue";
import IconMessages from "~vue/atoms/Icons/IconMessages.vue";
import IconPassword from "~vue/atoms/Icons/IconPassword.vue";
import IconServerConnect from "~vue/atoms/Icons/IconServerConnect.vue";
import IconSettings from "~vue/atoms/Icons/IconSettings.vue";
import IconTrashDelete from "~vue/atoms/Icons/IconTrashDelete.vue";
import IconUser from "~vue/atoms/Icons/IconUser.vue";
import IconValidated from "~vue/atoms/Icons/IconValidated.vue";
import IconVisualPassword from "~vue/atoms/Icons/IconVisualPassword.vue";

import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { capitalize } from "@phisyrc/std/str/capitalize";

onMounted(() => {
	document.documentElement.dataset["js"] = "on";
	document.title = "Design System | phisyRC";
});

const route = useRoute();

const props = defineProps(["dyncomponent"]);

const load_component = computed(() => {
	return `Story${capitalize(props.dyncomponent)}`;
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
	{
		text: "Utilitaires",
		link: "/ui/utilities",
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
	{ icon: IconMessages, text: "Messages" },
	{ icon: IconPassword, text: "Password" },
	{ icon: IconServerConnect, text: "ServerConnect" },
	{ icon: IconSettings, text: "Settings" },
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
			<h1 class="[ heading=6 align-t:center ]">
				Design System pour phisyRC
			</h1>

			<aside class="[ flex:full scroll:y scroll:hidden ]">
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
			</aside>
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

				<section
					class="app:bg=design-system [ flex:full scroll:y border:radius=2 ]"
				>
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
