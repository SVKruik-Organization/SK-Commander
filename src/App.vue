<script setup>
import NavBar from './components/NavBar.vue';

</script>

<template>
	<NavBar></NavBar>
	<router-view :key="$route.path"></router-view>
</template>

<style scoped></style>
