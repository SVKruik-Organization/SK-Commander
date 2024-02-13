<script lang='js'>

export default {
    name: "GeneralSettings",
    props: ["guild"],
    watch: {
        guild(newGuild) {
            this.$store.dispatch("setActiveGuild", newGuild);
        }
    },
    methods: {
        save(inputId, iconId) {
            const input = document.getElementById(inputId);
            if (!input) return;
            const icon = document.getElementById(iconId);
            if (!icon) return;
            icon.classList.remove("visible");

            // TODO - Database Update
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
                <p class="input-label text-shadow">Admin</p>
                <div class="input-wrapper">
                    <input :value="this.guild.channel_admin" type="text" class="shadow input" id="channelIdAdmin"
                        @change="this.change('save-channelIdAdmin')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdAdmin"
                        @click="this.save('channelIdAdmin', 'save-channelIdAdmin')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Event</p>
                <div class="input-wrapper">
                    <input :value="this.guild.channel_event" type="text" class="shadow input" id="channelIdEvent"
                        @change="this.change('save-channelIdEvent')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdEvent"
                        @click="this.save('channelIdEvent', 'save-channelIdEvent')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Suggestion</p>
                <div class="input-wrapper">
                    <input :value="this.guild.channel_suggestion" type="text" class="shadow input" id="channelIdSuggestion"
                        @change="this.change('save-channelIdSuggestion')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdSuggestion"
                        @click="this.save('channelIdSuggestion', 'save-channelIdSuggestion')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Snippet</p>
                <div class="input-wrapper">
                    <input :value="this.guild.channel_snippet" type="text" class="shadow input" id="channelIdSnippet"
                        @change="this.change('save-channelIdSnippet')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdSnippet"
                        @click="this.save('channelIdSnippet', 'save-channelIdSnippet')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Rules</p>
                <div class="input-wrapper">
                    <input :value="this.guild.channel_rules" type="text" class="shadow input" id="channelIdRules"
                        @change="this.change('save-channelIdRules')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-channelIdRules"
                        @click="this.save('channelIdRules', 'save-channelIdRules')"></i>
                </div>
            </div>
        </section>
        <section class="settings-item">
            <p class="header text-shadow">Roles</p>
            <div class="input-container">
                <p class="input-label text-shadow">Blinded ID</p>
                <div class="input-wrapper">
                    <input :value="this.guild.role_blinded" type="text" class="shadow input" id="roleIdBlinded" @change="this.change('save-roleIdBlinded')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-roleIdBlinded"
                        @click="this.save('roleIdBlinded', 'save-roleIdBlinded')"></i>
                </div>
            </div>
        </section>
        <section class="settings-item">
            <p class="header text-shadow">Other</p>
            <div class="input-container">
                <p class="input-label text-shadow">Welcome Message</p>
                <div class="input-wrapper">
                    <div class="select-wrapper shadow input">
                        <select :value="this.guild.welcome" id="welcomeMessage" @change="this.change('save-welcomeMessage')">
                            <option value="true">Enabled</option>
                            <option value="false">Disabled</option>
                        </select>
                        <i class="fa-solid fa-caret-down select-icon"></i>
                    </div>
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-welcomeMessage"
                        @click="this.save('welcomeMessage', 'save-welcomeMessage')"></i>
                </div>
            </div>
        </section>
    </div>
</template>

<style scoped>
.select-wrapper {
    width: 160px;
    height: 25px;
    border-radius: var(--border-radius-low);
    background-color: var(--fill-mid);
    border: 2px solid #FFFFFF10;
}

select {
    width: 150px;
}
</style>
