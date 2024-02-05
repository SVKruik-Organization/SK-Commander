<script lang='js'>

export default {
    name: "GeneralSettings",
    methods: {
        save(inputId, iconId) {
            const input = document.getElementById(inputId);
            if (!input) return;
            const icon = document.getElementById(iconId);
            if (!icon) return;

            icon.classList.remove("visible");
        },
        change(iconId) {
            const icon = document.getElementById(iconId);
            if (!icon) return;

            icon.classList.add("visible");
        }
    }
};
</script>

<template>
    <div class="content-container">
        <section class="settings-item">
            <p class="header text-shadow">Channel IDs</p>
            <div class="input-container">
                <p class="input-label text-shadow" id="channelIdAdmin">Admin</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-channelIdAdmin')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdAdmin"
                        @click="this.save('channelIdAdmin', 'save-channelIdAdmin')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow" id="channelIdEvent">Event</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-channelIdEvent')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdEvent"
                        @click="this.save('channelIdEvent', 'save-channelIdEvent')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow" id="channelIdSuggestion">Suggestion</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-channelIdSuggestion')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdSuggestion"
                        @click="this.save('channelIdSuggestion', 'save-channelIdSuggestion')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow" id="channelIdSnippet">Snippet</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-channelIdSnippet')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdSnippet"
                        @click="this.save('channelIdSnippet', 'save-channelIdSnippet')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow" id="channelIdRules">Rules</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-channelIdRules')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdRules"
                        @click="this.save('channelIdRules', 'save-channelIdRules')"></i>
                </div>
            </div>
        </section>
        <section class="settings-item">
            <p class="header text-shadow">Roles</p>
            <div class="input-container">
                <p class="input-label text-shadow" id="roleIdBlinded">Blinded ID</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-roleIdBlinded')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-roleIdBlinded"
                        @click="this.save('roleIdBlinded', 'save-roleIdBlinded')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow" id="roleCosmeticPosition">Cosmetic Position</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-roleCosmeticPosition')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-roleCosmeticPosition"
                        @click="this.save('roleCosmeticPosition', 'save-roleCosmeticPosition')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow" id="roleLevelPosition">Level Position</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-roleLevelPosition')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-roleLevelPosition"
                        @click="this.save('roleLevelPosition', 'save-roleLevelPosition')"></i>
                </div>
            </div>
        </section>
        <section class="settings-item">
            <p class="header text-shadow">Other</p>
            <div class="input-container">
                <p class="input-label text-shadow" id="welcomeMsg">Welcome Msg</p>
                <div class="input-wrapper">
                    <input type="text" class="shadow" @change="this.change('save-welcomeMsg')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-welcomeMsg"
                        @click="this.save('welcomeMsg', 'save-welcomeMsg')"></i>
                </div>
            </div>
        </section>
    </div>
</template>

<style scoped>
.content-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
    width: 100%;
    overflow-y: auto;
    height: 400px;
}

.settings-item {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.header {
    color: var(--accent);
    font-weight: bold;
}

.input-container {
    display: flex;
    width: 100%;
    justify-content: space-between;
    align-items: center;
}

input {
    width: 160px;
    height: 25px;
    border-radius: var(--border-radius-low);
    background-color: var(--fill-light);
    border: 2px solid #FFFFFF10;
}

.input-wrapper {
    display: flex;
    align-items: center;
    gap: 5px;
    margin-right: 5px;
}

.save-icon {
    color: var(--font-light);
    cursor: pointer;
    opacity: 0;
}

.visible {
    opacity: 1;
}
</style>
