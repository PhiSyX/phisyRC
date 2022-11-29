<script lang="ts" setup>
import type { Props as LoginFormProps } from "~/organisms/LoginForm/props";
import { ref } from "vue";
import { Some, uuid } from "@phisyrc/std";

import ChatLoginForm from "./LoginForm.vue";
import ChatLoginDialog from "./LoginDialog.vue";

let nickname = ref("PhiSyX");
let server_password = ref("");
let channels = ref<LoginFormProps["channels"]>([
	{
		id: uuid(),
		name: "#irc",
		topic: "Topic #irc",
		is_bookmarked: true,
		is_checked: false,
		image_url: Some("https://picsum.photos/200"),
	},
	{
		id: uuid(),
		name: "#ibug",
		topic: "Topic #ibug",
		is_bookmarked: false,
		is_checked: false,
		image_url: Some("https://picsum.photos/300"),
	},
]);
</script>

<template>
	<section class="[ flex! gap=4 p=1 ]">
		<ChatLoginForm
			v-model:nickname="nickname"
			v-model:server_password="server_password"
			v-model:channels="channels"
		/>

		<ChatLoginDialog id="chat-login-dlg" v-model="channels" />

		<div style="color: white">
			<div>Nickname: {{ nickname }}</div>
			<div>Server Password: {{ server_password }}</div>
			<div>Channels: {{ channels }}</div>
		</div>
	</section>
</template>

<style scoped>
#chat-login-dlg {
	margin-top: 0 !important;
}
</style>
