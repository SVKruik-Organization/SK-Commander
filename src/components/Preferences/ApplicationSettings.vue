<script lang='js'>
import { getVersion } from '@tauri-apps/api/app';
export default {
    name: "ApplicationSettings",
    props: {
        user: Object,
        guilds: Array
    },
    data() {
        return {
            version: "",
            button: null
        }
    },
    emits: [
        "popUpTransfer"
    ],
    async mounted() {
        this.version = `v${await getVersion()} Stable`;
        this.button = document.getElementsByClassName("settings-save-button")[0]
    },
    methods: {
        /**
         * Check for updates on remote.
         */
        updateCheck() {
            const icon = document.getElementsByClassName("loading-icon")[0];
            icon.classList.add("visible");
            setTimeout(() => {
                icon.classList.remove("visible");
            }, 1000);
        },
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
            this.$emit("popUpTransfer", "success", "Application information saved.", 4000);
        }
    }
};
</script>

<template>
    <div class="content-container">
        <section class="settings-component">
            <p class="header text-shadow">Notifications</p>
            <div class="setting-item">
                <p class="setting-name">Update Available</p>
                <input @change="this.unlockButton()" type="checkbox">
            </div>
        </section>
        <section class="settings-component">
            <p class="header text-shadow">Appearance</p>
            <div class="input-container">
                <p class="input-label text-shadow">Theme</p>
                <div class="input-wrapper">
                    <div class="select-wrapper shadow input">
                        <select @change="this.unlockButton()">
                            <option value="dark">Dark</option>
                            <option value="light">Light</option>
                            <option value="system">System</option>
                        </select>
                        <i class="fa-solid fa-caret-down select-icon"></i>
                    </div>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Language</p>
                <div class="input-wrapper">
                    <div class="select-wrapper shadow input">
                        <select @change="this.unlockButton()">
                            <option value="en">English</option>
                            <option value="nl">Nederlands</option>
                            <option value="en">More languages WIP</option>
                        </select>
                        <i class="fa-solid fa-caret-down select-icon"></i>
                    </div>
                </div>
            </div>
        </section>
        <section class="settings-component">
            <p class="header text-shadow">Information</p>
            <div class="setting-item">
                <p class="setting-name">Version</p>
                <p>{{ this.version }}</p>
            </div>
            <div class="setting-item">
                <p class="setting-name">Package</p>
                <p>{{ this.user.edition }}</p>
            </div>
        </section>
        <div class="settings-component">
            <section class="settings-nested-component">
                <button @click="this.save()" class="settings-save-button">Save Changes</button>
            </section>
            <section class="settings-nested-component">
                <div class="version-items">
                    <div class="update-check-container">
                        <i class="fa-solid fa-spinner loading-icon"></i>
                        <button @click="this.updateCheck()" class="setting-name faded-text update-check smaller">Check for
                            updates</button>
                    </div>
                    <i class="fa-solid fa-circle faded-text dot"></i>
                    <a href="https://github.com/SVKruik/bot-config-ui/releases" class="faded-text smaller"
                        target="_blank">Release Notes</a>
                    <i class="fa-solid fa-circle faded-text dot"></i>
                    <a href="https://github.com/SVKruik/bot-config-ui/releases" class="faded-text smaller"
                        target="_blank">Bèta
                        Version</a>
                </div>
            </section>
        </div>
    </div>
</template>

<style scoped>
.content-container {
    box-sizing: border-box;
    padding-right: 20px;
    overflow: visible;
    scrollbar-gutter: stable;
}

.select-wrapper {
    width: 160px;
    height: 25px;
    border-radius: var(--border-radius-low);
    background-color: var(--fill-mid);
    border: 2px solid var(--font-light);
}

select {
    width: 150px;
}

.update-check-container {
    display: flex;
    align-items: center;
    gap: 5px;
}

.update-check {
    background-color: transparent;
}

.version-items {
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.settings-component:last-child {
    margin-top: auto;
    gap: 15px;
}

.settings-nested-component:last-child {
    margin-bottom: 30px;
    width: 320px;
    margin-left: -15px;
}
</style>
