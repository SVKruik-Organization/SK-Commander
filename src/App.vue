<script lang='js'>
import NavBar from './components/NavBar.vue';
import InformationOverlay from './components/InformationOverlay.vue';

export default {
	name: "App",
	components: { NavBar, InformationOverlay },
	data() {
		return {
			user: null,
			guilds: [],
			informationOverlayStatus: false,
		};
	},
	methods: {
		/**
		 * Setup the user account.
		 * @param {Object} user User Object From the Stelleri API.
		 * @param {Array} guilds The Guilds this operator controls.
		 */
		login(user, guilds) {
			this.user = user;
			this.guilds = guilds;
			this.$router.push("/home/general");
		},
		/**
		 * Clear session and redirect to login.
		 */
		logout() {
			this.user = null;
			this.guilds = [];
			this.$router.push("/");
		}
	}
};
</script>

<template>
	<InformationOverlay v-if="this.informationOverlayStatus"
		@toggleInformationOverlay="this.informationOverlayStatus = !this.informationOverlayStatus"></InformationOverlay>
	<NavBar :user="this.user"></NavBar>
	<router-view :key="$route.path" :user="this.user" :guilds="this.guilds" @login="this.login" @logout="this.logout"
		@toggleInformationOverlay="this.informationOverlayStatus = !this.informationOverlayStatus"></router-view>
</template>

<style scoped></style>
