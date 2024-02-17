<script lang='js'>
export default {
    name: "PreferencesPage",
    props: {
        user: Object,
        guilds: Array
    },
    created() {
        if (!this.user) {
            return this.$router.push("/unauthorized");
        } else if (new Date(this.user.exp * 1000) < new Date()) return this.$router.push("/session-expired");
    },
    emits: [
        "popup"
    ],
    methods: {
        popUpTransfer(type, content, time) {
            this.$emit("popup", type, content, time);
        }
    }
};
</script>

<template>
    <div class="content">
        <div class="content-wrapper">
            <section class="settings-container">
                <div class="menu-list">
                    <router-link class="menu-item text-shadow" to="/preferences/account">Account</router-link>
                    <span class="splitter shadow"></span>
                    <router-link class="menu-item text-shadow" to="/preferences/application">Application</router-link>
                    <span class="splitter shadow"></span>
                    <router-link class="menu-item text-shadow" to="/preferences/links">Links</router-link>
                </div>
                <span class="splitter-accent"></span>
                <router-view :guilds="this.guilds" :user="this.user" @popUpTransfer="this.popUpTransfer"></router-view>
            </section>
        </div>
    </div>
</template>

<style scoped>
.content-wrapper {
    background-color: var(--fill);
    border-radius: var(--border-radius-high);
    height: 440px;
    box-sizing: border-box;
    padding: 0 0 35px 35px;
    position: relative;
}
</style>
