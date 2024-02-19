<script lang='js'>
export default {
    name: "AccountPreferences",
    props: {
        user: Object,
        guilds: Array
    },
    data() {
        return {
            button: null
        }
    },
    emits: [
        "popUpTransfer"
    ],
    async mounted() {
        this.button = document.getElementsByClassName("settings-save-button")[0]
    },
    methods: {
        /**
         * Show the button on input change.
         */
        unlockButton() {
            if (this.button) this.button.classList.add("visible");
        },
        /**
         * Persist modified preferences.
         */
        save() {
            if (!this.button || !this.button.classList.contains("visible")) return;
            this.button.classList.remove("visible");
            this.$emit("popUpTransfer", "success", "Account information saved.", 4000);
        }
    }
};
</script>

<template>
    <div class="content-container">
        <section class="settings-component">
            <p class="header text-shadow">General</p>
            <div class="setting-item">
                <p class="input-label text-shadow">Username</p>
                <input :value="this.user.operator_username" @change="this.unlockButton()" type="text" class="shadow input">
            </div>
            <div class="setting-item">
                <p class="input-label text-shadow">Email</p>
                <input :value="this.user.email" @change="this.unlockButton()" type="text" class="shadow input">
            </div>
            <div class="setting-item">
                <p class="input-label text-shadow">Service Tag</p>
                <input :value="this.user.service_tag" type="text" disabled class="shadow input">
            </div>
        </section>
        <section class="settings-component">
            <p class="header text-shadow">2FA Security</p>
            <div class="setting-item">
                <p class="input-label text-shadow">Email</p>
                <p>Not Setup</p>
            </div>
            <div class="setting-item">
                <p class="input-label text-shadow">Phone</p>
                <p>Not Setup</p>
            </div>
            <div class="setting-item">
                <p class="input-label text-shadow">Recovery Keys</p>
                <p>Not Setup</p>
            </div>
        </section>
        <section class="settings-component">
            <p class="header text-shadow">Billing</p>
            <div class="setting-item">
                <p class="input-label text-shadow">Active Plan</p>
                <input :value="this.user.edition" type="text" disabled class="shadow input">
            </div>
            <div class="setting-item">
                <span></span>
                <router-link to="/plans" class="small">{{ this.user.edition === 'Basic' ? 'Upgrade' : 'Change'
                }}</router-link>
            </div>
        </section>
        <section class="settings-component">
            <button @click="this.save()" class="settings-save-button">Save Changes</button>
        </section>
    </div>
</template>

<style scoped>
.content-container {
    box-sizing: border-box;
    padding-right: 20px;
    scrollbar-gutter: stable;
    overflow-x: hidden;
}

.settings-component:last-child {
    margin-top: auto;
    margin-bottom: 20px;
    width: 310px;
    gap: 15px;
}
</style>
