<script setup lang="ts">
import IconBookmark from "~vue/atoms/Icons/IconBookmark.vue";
import IconChecked from "~vue/atoms/Icons/IconChecked.vue";

import type { Props as LoginFormProps } from "~/organisms/LoginForm/props";
import { ref } from "vue";
import { use_model } from "~vue/hooks/use_models";
import { is_empty } from "@phisyrc/std/lang/is_empty";
import { uuid } from "@phisyrc/std/str/uuid";
import { None } from "@phisyrc/std";

type Props = {
	modelValue: LoginFormProps["channels"];
};

const props = defineProps<Props>();
const emit = defineEmits(["update:modelValue"]);
let channels$ = use_model(props)(emit);
let new_channel_name = ref("");

function add_channel_handler(evt: Event) {
	evt.preventDefault();

	let new_channel_name_stripped = new_channel_name.value.trim();

	if (
		is_empty(new_channel_name_stripped) ||
		new_channel_name_stripped.startsWith("#") === false
	) {
		return;
	}

	let new_channel = {
		id: uuid(),
		name: new_channel_name.value,
		topic: "",
		is_bookmarked: false,
		is_checked: false,
		image_url: None,
	};

	channels$.value.push(new_channel);
}
</script>

<template>
	<div
		class="dialog@channels dialog@channels:form<input> dialog@channels:form<submit> [ flex! gap=1 p=1 border:radius=2 mt=20 ]"
	>
		<form
			method="POST"
			class="[ flex align-i:center gap=1 h=4 ]"
			@submit="add_channel_handler"
		>
			<input
				class="[ input:reset flex:full h:full p=1 ]"
				type="text"
				v-model="new_channel_name"
			/>
			<button
				class="[ btn:reset h:full border:radius=1 f-family=roboto p=1 ]"
				type="submit"
			>
				Ajouter
			</button>
		</form>

		<ol
			class="dialog@channels:list<item> dialog@channels:list<bookmark> [ scroll:y scroll:hidden ][ flex! gap=1 h:full list:reset ]"
		>
			<li
				v-for="channel in channels$"
				:key="channel.id"
				class="[ flex align-i:center h:full gap=2 p=1 border:radius=2 ]"
				@click="channel.is_checked = !channel.is_checked"
			>
				<div class="dialog@channels:list<item:checked>">
					<IconChecked v-model="channel.is_checked" />
				</div>

				<div class="[ size=7 ]">
					<img
						v-if="channel.image_url.is_some()"
						:src="channel.image_url.unwrap()"
						:alt="channel.name"
						class="[ h=7 border:radius=8 border-br:radius=1 box:shadow ]"
					/>
				</div>

				<div
					class="pos-r [ flex:full h:full py=1 scroll:y scroll:hidden ]"
				>
					<strong>{{ channel.name }}</strong>
					<button
						type="button"
						class="[ btn:reset border:radius=1 ]"
						:class="{ 'is-bookmarked': channel.is_bookmarked }"
						@click.stop="
							channel.is_bookmarked = !channel.is_bookmarked
						"
					>
						<IconBookmark />
					</button>
					<p>{{ channel.topic }}</p>
				</div>
			</li>
		</ol>
	</div>
</template>

<style lang="scss">
@import "design/app/organisms/login-dialog";
</style>
