<script lang="ts">
export default {
	name: "LayoutDefault",
};
</script>

<script setup lang="ts">
import Overlayer from "~vue/organisms/Overlayer/Overlayer.vue";
</script>

<template>
	<RouterView />
	<Overlayer />
</template>
