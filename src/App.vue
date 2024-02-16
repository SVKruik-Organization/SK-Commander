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
			this.$router.push("/");
			this.user = null;
			this.guilds = [];
		},
		/**
		 * Check if the user is logged in. If not, redirect.
		 */
		checkStatus() {
			if (!this.user) {
				return this.$router.push("/unauthorized");
			} else if (new Date(this.user.exp * 1000) < new Date()) return this.$router.push("/session-expired");
		}
	}
};
</script>

<template>
	<InformationOverlay v-if="this.informationOverlayStatus"
		@toggleInformationOverlay="this.informationOverlayStatus = !this.informationOverlayStatus"></InformationOverlay>
	<NavBar :user="this.user" @logout="this.logout"></NavBar>
	<router-view :key="$route.path" :user="this.user" :guilds="this.guilds" @login="this.login" @logout="this.logout"
		@toggleInformationOverlay="this.informationOverlayStatus = !this.informationOverlayStatus"
		@checkStatus="this.checkStatus"></router-view>
</template>

<style scoped></style>
