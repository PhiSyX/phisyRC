<script lang="ts" setup>
import { ref } from "vue";

import ChatLoginForm from "~vue/organisms/LoginForm/LoginForm.vue";

let nickname = ref("PhiSyX");
let server_password = ref("");
let channels = ref(["#irc", "##ibug"]);
</script>

<template>
	<section class="[ flex gap=4 p=1 ]">
		<ChatLoginForm
			v-model:nickname="nickname"
			v-model:server_password="server_password"
			v-model:channels="channels"
		/>

		<div style="color: white">
			<div>Nickname: {{ nickname }}</div>
			<div>Server Password: {{ server_password }}</div>
			<div>Channels: {{ channels }}</div>
		</div>
	</section>
</template>
