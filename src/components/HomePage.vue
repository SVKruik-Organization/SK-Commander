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
        "toggleInformationOverlay",
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
                    <router-view :guild="this.activeGuild"></router-view>
                </section>
            </div>
        </div>
    </div>
</template>

<style scoped>
.select-container {
    display: flex;
    gap: 10px;
}

.settings-container-wrapper {
    background-color: var(--fill);
    border-radius: var(--border-radius-high);
    height: 440px;
    box-sizing: border-box;
    padding: 0 0 35px 35px;
    position: relative;
}

.information-icon-wrapper {
    cursor: pointer;
    position: absolute;
    right: 10px;
    top: 10px;
}

.information-icon {
    color: var(--accent);
}

.settings-container {
    display: flex;
    justify-content: space-between;
    margin-top: 35px;
}

.menu-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: 120px;
}

.menu-item {
    height: 32px;
    width: 120px;
    font-weight: bold;
    display: flex;
    align-items: center;
    box-sizing: border-box;
    padding-left: 10px;
}

.active {
    background-color: var(--fill-mid);
    color: var(--accent);
    border-radius: var(--border-radius-low);
    -moz-box-shadow: 0 4px 4px rgba(0, 0, 0, 0.1);
    -webkit-box-shadow: 0 4px 4px rgba(0, 0, 0, 0.1);
    box-shadow: 0 4px 4px rgba(0, 0, 0, 0.1);
}

.splitter {
    width: 120px;
    height: 2px;
}

.splitter-accent {
    height: 360px;
    width: 2px;
}
</style>
