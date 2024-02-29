<script lang='js'>
import NavBar from './components/NavBar.vue';
import InformationOverlay from './components/InformationOverlay.vue';
import NotificationPopup from './components/NotificationPopup.vue';
import { createTicket } from './assets/utils/ticket.js';

export default {
	name: "App",
	components: { NavBar, InformationOverlay, NotificationPopup },
	data() {
		return {
			user: null,
			guilds: [],
			informationOverlayStatus: false,
			popUpForbidden: ["/unauthorized", "/session-expired"],
			navBarForbidden: ["/", "/unauthorized", "/session-expired"],
			pendingPopups: []
		};
	},
	methods: {
		/**
		 * Setup the user account.
		 * @param {Object} user User Object From the Apricaria API.
		 * @param {Array} guilds The Guilds this operator controls.
		 */
		login(user, guilds) {
			this.user = user;
			this.guilds = guilds;
			this.$router.push("/home");
		},
		/**
		 * Clear session and redirect to login.
		 */
		logout() {
			this.$router.push("/");
		},
		/**
		 * Check if the user is logged in. If not, redirect.
		 */
		checkStatus() {
			if (!this.user) {
				return this.$router.push("/unauthorized");
			} else if (new Date(this.user.exp * 1000) < new Date()) return this.$router.push("/session-expired");
		},
		/**
		 * Show a popup to the user.
		 * @param {string} type The type of message. Correspondents to a color.
		 * @param {string} content The message content.
		 * @param {number} time Expiry time in milliseconds.
		 */
		popup(type, content, time) {
			const id = `popup${createTicket()}`;
			const childComponenet = this.$refs.notificationPopup;
			this.pendingPopups.push({
				"id": id,
				"color": `var(--${type})`,
				"message": content
			});

			setTimeout(() => {
				if (childComponenet) childComponenet.fadeIn(id);
			}, 10);

			setTimeout(() => {
				if (childComponenet) childComponenet.closePopup(id);
			}, time);
		}
	}
};
</script>

<template>
	<NotificationPopup ref="notificationPopup" :pendingPopups="this.pendingPopups" v-if="!this.popUpForbidden.includes(this.$route.path)">
	</NotificationPopup>
	<InformationOverlay v-if="this.informationOverlayStatus"
		@toggleInformationOverlay="this.informationOverlayStatus = !this.informationOverlayStatus"></InformationOverlay>
	<NavBar v-if="!this.navBarForbidden.includes(this.$route.path)" :user="this.user" @logout="this.logout"></NavBar>
	<router-view :key="$route.path" :user="this.user" :guilds="this.guilds" @login="this.login" @logout="this.logout"
		@toggleInformationOverlay="this.informationOverlayStatus = !this.informationOverlayStatus"
		@checkStatus="this.checkStatus" @popup="this.popup"></router-view>
</template>

<style scoped></style>
