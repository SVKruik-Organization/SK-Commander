<script lang='js'>
export default {
    name: "HomePage",
    computed: {
        activeGuild() {
            return this.$store.getters.getActiveGuild;
        }
    },
    props: {
        user: Object,
        guilds: Array,
    },
    emits: [
        "toggleInformationOverlay"
    ],
    created() {
        if (!this.user) {
            return this.$router.push("/unauthorized");
        } else if (new Date(this.user.exp * 1000) < new Date()) return this.$router.push("/session-expired");
        if (!this.activeGuild) this.$store.commit("setActiveGuild", this.guilds[0]);
    },
    mounted() {
        if (this.activeGuild) this.$refs["serverSelect"].value = this.activeGuild.snowflake;
    },
    methods: {
        /**
         * Change the selected guild.
         * @param {Object} event DOM Change Event.
         */
        changeGuild(event) {
            const selectedGuild = this.guilds.find(guild => guild.snowflake === event.target.value);
            if (selectedGuild) {
                this.$store.commit("setActiveGuild", selectedGuild);
                this.initialGuild = null;
            }
        }
    },
};
</script>

<template>
    <div class="content">
        <div class="content-wrapper">
            <section class="select-container">
                <div class="select-wrapper shadow">
                    <select ref="serverSelect" @change="this.changeGuild($event);">
                        <option v-for="guild in this.guilds" :value="guild.snowflake">{{ guild.name }}</option>
                    </select>
                    <i class="fa-solid fa-caret-down select-icon"></i>
                </div>
            </section>
            <div class="settings-container-wrapper">
                <div class="information-icon-wrapper" @click="this.$emit('toggleInformationOverlay');"><i
                        class="fa-solid fa-circle-info information-icon"></i></div>
                <section class="settings-container">
                    <div class="menu-list">
                        <router-link class="menu-item text-shadow" to="/home/general">General</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item text-shadow" to="/home/economy">Economy</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item text-shadow" to="/home/level">Level</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item text-shadow" to="/home/users">Users</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item text-shadow" to="/home/information">Information</router-link>
                    </div>
                    <span class="splitter-accent"></span>
                    <router-view :guild="this.activeGuild" :user="this.user"></router-view>
                </section>
            </div>
        </div>
    </div>
</template>

<style scoped></style>