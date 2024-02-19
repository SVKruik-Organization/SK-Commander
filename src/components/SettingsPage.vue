<script lang='js'>
export default {
    name: "SettingsPage",
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
            <div class="settings-container-wrapper shadow">
                <div class="information-icon-wrapper" @click="this.$emit('toggleInformationOverlay');"><i
                        class="fa-solid fa-circle-info information-icon"></i></div>
                <section class="settings-container">
                    <div class="menu-list">
                        <router-link class="menu-item bold text-shadow" to="/settings/general">General</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item bold text-shadow" to="/settings/economy">Economy</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item bold text-shadow" to="/settings/level">Level</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item bold text-shadow" to="/settings/users">Users</router-link>
                        <span class="splitter shadow"></span>
                        <router-link class="menu-item bold text-shadow" to="/settings/information">Information</router-link>
                    </div>
                    <span class="splitter-accent"></span>
                    <router-view :guild="this.activeGuild" :user="this.user"></router-view>
                </section>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Server Selection */
.select-container {
    display: flex;
    gap: 10px;
}

/* Settings */
.settings-container-wrapper {
    background-color: var(--fill);
    border-radius: var(--border-radius-high);
    height: 440px;
    box-sizing: border-box;
    padding: 0 0 35px 35px;
    position: relative;
}

.information-icon-wrapper {
    position: absolute;
    right: 10px;
    top: 10px;
}

.information-icon {
    color: var(--accent);
    cursor: pointer;
}
</style>