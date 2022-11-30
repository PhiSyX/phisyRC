<script lang="ts">
import { defineAsyncComponent } from "vue";

// Générales
const StoryColors = defineAsyncComponent(
	() => import("~vue/stories/StoryColors.vue")
);
const StoryTypography = defineAsyncComponent(
	() => import("~vue/stories/StoryTypography.vue")
);
const StoryUtilities = defineAsyncComponent(
	() => import("~vue/stories/StoryUtilities.vue")
);

// Atoms
const StoryButton = defineAsyncComponent(
	() => import("~vue/atoms/Button/Story.vue")
);

// Molecules
const StorySidebarItem = defineAsyncComponent(
	() => import("~vue/molecules/SidebarItem/Story.vue")
);
const StorySidebarList = defineAsyncComponent(
	() => import("~vue/molecules/SidebarList/Story.vue")
);
const StoryTopic = defineAsyncComponent(
	() => import("~vue/molecules/Topic/Story.vue")
);

// Organisms
const StoryChatLoginForm = defineAsyncComponent(
	() => import("~vue/organisms/LoginForm/Story.vue")
);
const StorySidebar = defineAsyncComponent(
	() => import("~vue/organisms/Sidebar/Story.vue")
);

export default {
	name: "UI",
	components: {
		// Générales
		StoryColors,
		StoryTypography,
		StoryUtilities,

		// Atoms
		StoryButton,
		// Molecules
		StorySidebarItem,
		StorySidebarList,
		StoryTopic,
		// Organisms
		StoryChatLoginForm,
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
import IconAttachFile from "~vue/atoms/Icons/IconAttachFile.vue";
import IconBookmark from "~vue/atoms/Icons/IconBookmark.vue";
import IconChannel from "~vue/atoms/Icons/IconChannel.vue";
import IconChecked from "~vue/atoms/Icons/IconChecked.vue";
import IconColor from "~vue/atoms/Icons/IconColor.vue";
import IconCross from "~vue/atoms/Icons/IconCross.vue";
import IconMessage from "~vue/atoms/Icons/IconMessage.vue";
import IconMessageEmpty from "~vue/atoms/Icons/IconMessageEmpty.vue";
import IconMessages from "~vue/atoms/Icons/IconMessages.vue";
import IconNicklist from "~vue/atoms/Icons/IconNicklist.vue";
import IconPassword from "~vue/atoms/Icons/IconPassword.vue";
import IconSendMessage from "~vue/atoms/Icons/IconSendMessage.vue";
import IconServerConnect from "~vue/atoms/Icons/IconServerConnect.vue";
import IconSettings from "~vue/atoms/Icons/IconSettings.vue";
import IconSettingsVertical from "~vue/atoms/Icons/IconSettingsVertical.vue";
import IconTrashDelete from "~vue/atoms/Icons/IconTrashDelete.vue";
import IconUser from "~vue/atoms/Icons/IconUser.vue";
import IconValidated from "~vue/atoms/Icons/IconValidated.vue";
import IconVisualPassword from "~vue/atoms/Icons/IconVisualPassword.vue";
import IconVoiceRecording from "~vue/atoms/Icons/IconVoiceRecording.vue";

import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { capitalize } from "@phisyrc/std";

onMounted(() => {
	document.documentElement.dataset["js"] = "on";
	document.title = "Design System | phisyRC";
});

const route = useRoute();

const props = defineProps(["dyncomponent"]);

const load_component = computed(() => {
	return `Story${capitalize(props.dyncomponent, {
		includes_separators: false,
	})}`;
});

type Section = {
	opened?: boolean;
	title: string;
	components: List[];
};

type List = {
	icon?: unknown;
	text: string;
	link?: string;
};

let general: Section = {
	title: "Générale",
	components: [
		{
			icon: IconColor,
			text: "Les couleurs",
			link: "/design-system/colors",
		},
		{
			text: "Typographie",
			link: "/design-system/typography",
		},
		{
			text: "Utilitaires",
			link: "/design-system/utilities",
		},
	],
};

let icons: Section = {
	title: "Les icônes",
	opened: false,
	components: [
		{ icon: IconAdd, text: "Add" },
		{ icon: IconArrowDown, text: "ArrowDown" },
		{ icon: IconArrowLeft, text: "ArrowLeft" },
		{ icon: IconArrowRight, text: "ArrowRight" },
		{ icon: IconArrowUp, text: "ArrowUp" },
		{ icon: IconAttachFile, text: "AttachFile" },
		{ icon: IconBookmark, text: "Bookmark" },
		{ icon: IconChannel, text: "Channel" },
		{ icon: IconChecked, text: "Checked" },
		{ icon: IconCross, text: "Cross" },
		{ icon: IconMessage, text: "Message" },
		{ icon: IconMessageEmpty, text: "MessageEmpty" },
		{ icon: IconMessages, text: "Messages" },
		{ icon: IconNicklist, text: "Nicklist" },
		{ icon: IconPassword, text: "Password" },
		{ icon: IconSendMessage, text: "SendMessage" },
		{ icon: IconServerConnect, text: "ServerConnect" },
		{ icon: IconSettings, text: "Settings" },
		{ icon: IconSettingsVertical, text: "Settings (barre verticale)" },
		{ icon: IconTrashDelete, text: "TrashDelete" },
		{ icon: IconUser, text: "User" },
		{ icon: IconValidated, text: "Validated" },
		{ icon: IconVisualPassword, text: "VisualPassword" },
		{ icon: IconVoiceRecording, text: "VoiceRecording" },
	],
};

let atoms: Section = {
	title: "Les atomes",
	components: [
		{
			text: "Button",
			link: "/design-system/button",
		},
	],
};

let molecules: Section = {
	title: "Les molécules",
	components: [
		{
			text: "Barre latérale (élément)",
			link: "/design-system/sidebar-item",
		},

		{
			text: "Barre latérale (liste)",
			link: "/design-system/sidebar-list",
		},

		{
			text: "Topic",
			link: "/design-system/topic",
		},
	],
};

let organisms: Section = {
	title: "Les organismes",
	components: [
		{
			text: "Barre latérale",
			link: "/design-system/sidebar",
		},
		{
			text: "Formulaire de connexion au Chat",
			link: "/design-system/chat-login-form",
		},
	],
};

let sections: Section[] = [general, atoms, molecules, organisms, icons];
</script>

<template>
	<div id="ui-page" class="[ flex size:full gap=1 p=1 ]">
		<nav class="[ flex! p=1 border:radius=1 f-family=roboto ]">
			<h1 class="[ heading=6 align-t:center ]">
				Design System pour phisyRC
			</h1>

			<aside class="[ flex:full scroll:y scroll:hidden ]">
				<details
					v-for="section in sections"
					:open="section.opened ?? true"
				>
					<summary class="[ pl=1 pb=1 ]">{{ section.title }}</summary>

					<ul class="[ flex! gap=1 list:reset ]">
						<li
							class="[ pos-r flex align-i:center gap=1 px=1 border:radius=2 ]"
							v-for="item in section.components"
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
	min-width: space(320);
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
