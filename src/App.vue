<script lang='js'>
import NavBar from './components/NavBar.vue';

export default {
	name: "App",
	components: { NavBar },
	data() {
		return {
			jwt: null,
			guilds: []
		};
	},
	methods: {
		/**
		 * Setup the user account.
		 * @param {string} token JWT token from the Stelleri API.
		 * @param {Array} guilds The Guilds this operator controls.
		 */
		login(token, guilds) {
			this.jwt = token;
			this.guilds = guilds;
			this.$router.push("/home/general");
		},
		logout() {
			this.jwt = null;
			this.guilds = [];
		}
	}
};
</script>

<template>
	<NavBar></NavBar>
	<router-view :key="$route.path" :jwt="this.jwt" :guilds="this.guilds" @login="this.login" @logout="this.logout"></router-view>
</template>

<style scoped></style>
