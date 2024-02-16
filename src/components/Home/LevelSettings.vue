<script lang='js'>

export default {
    name: "LevelSettings",
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

            // Role Color Special
            if (inputId === "levelRoleColor") document.getElementsByClassName("role-color-preview")[0].style.backgroundColor = `#${input.value}`;

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
            <p class="header text-shadow">Level Role</p>
            <div class="input-container">
                <p class="input-label text-shadow">Position</p>
                <div class="input-wrapper">
                    <input :value="this.guild.role_level_power" type="number" min="4" class="shadow input" id="levelRolePosition"
                        @change="this.change('save-levelRolePosition')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-levelRolePosition"
                        @click="this.save('levelRolePosition', 'save-levelRolePosition')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Max</p>
                <div class="input-wrapper">
                    <input :value="this.guild.role_level_max" type="number" class="shadow input" id="levelRoleMax"
                        @change="this.change('save-levelRoleMax')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-levelRoleMax"
                        @click="this.save('levelRoleMax', 'save-levelRoleMax')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">HEX Color</p>
                <div class="input-wrapper role-color-wrapper">
                    <span class="role-color-preview" :style="`background-color: #${this.guild.role_level_color};`"></span>
                    <input ref="role_color" :value="this.guild.role_level_color" type="text" class="shadow input" id="levelRoleColor"
                        @change="this.change('save-levelRoleColor')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-levelRoleColor"
                        @click="this.save('levelRoleColor', 'save-levelRoleColor')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Status</p>
                <div class="input-wrapper">
                    <div class="select-wrapper shadow input">
                        <select :value="this.guild.role_level_enable" id="levelRoleEnable"
                            @change="this.change('save-levelRoleEnable')">
                            <option value="true">Enabled</option>
                            <option value="false">Disabled</option>
                        </select>
                        <i class="fa-solid fa-caret-down select-icon"></i>
                    </div>
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-levelRoleEnable"
                        @click="this.save('levelRoleEnable', 'save-levelRoleEnable')"></i>
                </div>
            </div>
        </section>
        <section class="settings-item">
            <p class="header text-shadow">Rewards</p>
            <div class="input-container">
                <p class="input-label text-shadow">Normal Message</p>
                <div class="input-wrapper">
                    <input :value="this.guild.xp_increase_normal" type="number" class="shadow input"
                        id="rewardNormalMessage" @change="this.change('save-rewardNormalMessage')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-rewardNormalMessage"
                        @click="this.save('rewardNormalMessage', 'save-rewardNormalMessage')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Slash Command</p>
                <div class="input-wrapper">
                    <input :value="this.guild.xp_increase_slash" type="number" class="shadow input" id="rewardSlashMessage"
                        @change="this.change('save-rewardSlashMessage')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-rewardSlashMessage"
                        @click="this.save('rewardSlashMessage', 'save-rewardSlashMessage')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Shop Purchase</p>
                <div class="input-wrapper">
                    <input :value="this.guild.xp_increase_purchase" type="number" class="shadow input" id="rewardPurchase"
                        @change="this.change('save-rewardPurchase')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-rewardPurchase"
                        @click="this.save('rewardPurchase', 'save-rewardPurchase')"></i>
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

.role-color-preview {
    width: 15px;
    height: 15px;
    border-radius: 50%;
}
</style>
