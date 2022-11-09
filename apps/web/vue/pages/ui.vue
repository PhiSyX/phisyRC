<script lang="ts">
import { defineAsyncComponent } from "vue";
import { useRoute } from "vue-router";

export default {
	name: "UI",
	components: {
	},
};
</script>

<script lang="ts" setup>
import { capitalize, computed, onMounted } from "vue";

onMounted(() => {
	document.documentElement.dataset["js"] = "on";
});

const route = useRoute();

const props = defineProps(["dyncomponent"]);

const load_component = computed(() => {
	return capitalize(props.dyncomponent) + "Story";
});

type List = {
	text: string;
	link: string;
};

let components: List[] = [
];

let icons: List[] = [];

let application: List[] = [];
</script>

<template>
	<div id="ui-page" class="[ flex size:full gap=1 p=1 ]">
		<nav class="[ flex! p=1 border:radius=1 f-family=roboto ]">
			<p class="[ align-t:center text:bold ]">
				Design System pour phisyRC
			</p>

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
						<span class="[ flex:full ]">{{ item.text }}</span>

						<RouterLink
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
						<span class="[ flex:full ]">{{ item.text }}</span>

						<RouterLink
							:to="item.link"
							class="pos-a:full"
						></RouterLink>
					</li>
				</ul>
			</details>

			<details>
				<summary class="[ pl=1 pb=1 ]">Application</summary>

				<ul class="[ flex! gap=1 list:reset scroll:y ]">
					<li
						class="[ pos-r flex align-i:center gap=1 px=1 border:radius=2 ]"
						v-for="item in application"
					>
						<span class="[ flex:full ]">{{ item.text }}</span>

						<RouterLink
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

				<section class="[ flex:full ]" data-theme="dark">
					<component :is="load_component">
						Sélectionner un composant
					</component>
				</section>

				<section class="[ flex:full ]" data-theme="light">
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

[data-theme="dark"] {
	background-color: var(--color-grey900);
	color: var(--color-white);
}

[data-theme="light"] {
	background-color: var(--color-white);
	color: var(--color-black);
}

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

section {
	@include --theme using($name) {
		@if $name == dark {
			border: 1px solid var(--color-grey);
		} @else if $name == light {
			border: 1px solid var(--color-grey400);
		}
	}
}
</style>
