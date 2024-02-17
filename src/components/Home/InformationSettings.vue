<script lang='js'>
export default {
    name: "InformationSettings",
    props: ["guild"],
    watch: {
        guild(newGuild) {
            this.$store.dispatch("setActiveGuild", newGuild);
        }
    }
};
</script>

<template>
    <div class="content-container">
        <section class="settings-component">
            <p class="header text-shadow">Server</p>
            <div class="input-container">
                <p class="input-label text-shadow">ID</p>
                <input :value="this.guild.snowflake" type="text" disabled class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Name</p>
                <input :value="this.guild.name" type="text" disabled class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Locale</p>
                <input :value="this.guild.locale" type="text" disabled class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Status</p>
                <div class="input-wrapper">
                    <div class="select-wrapper shadow input">
                        <select :value="this.guild.disabled" disabled>
                            <option value="false">Enabled</option>
                            <option value="true">Disabled</option>
                        </select>
                        <i class="fa-solid fa-caret-down select-icon"></i>
                    </div>
                </div>
            </div>
        </section>
        <section class="settings-component">
            <p class="header text-shadow">Dates</p>
            <div class="input-container">
                <p class="input-label text-shadow">Creation</p>
                <input :value="new Date(this.guild.guild_date_creation).toLocaleString()" type="text" disabled
                    class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Last Update</p>
                <input
                    :value="this.guild.guild_date_update ? new Date(this.guild.guild_date_update).toLocaleString() : 'Never'"
                    type="text" disabled class="shadow input">
            </div>
        </section>
        <section class="settings-component">
            <p class="header text-shadow">Operator</p>
            <div class="input-container">
                <p class="input-label text-shadow">ID</p>
                <input :value="this.guild.operator_id" type="text" disabled class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Name</p>
                <input :value="this.guild.operator_name" type="text" disabled class="shadow input">
            </div>
        </section>
    </div>
</template>

<style scoped>
.input {
    margin-right: 30px;
}

.checkbox,
.input-wrapper {
    margin-right: 30px;
}

.select-wrapper {
    width: 160px;
    height: 25px;
    border-radius: var(--border-radius-low);
    background-color: var(--fill-mid);
    border: 2px solid var(--font-light);
    margin-right: 0;
}

select {
    width: 150px;
    cursor: default;
}
</style>
