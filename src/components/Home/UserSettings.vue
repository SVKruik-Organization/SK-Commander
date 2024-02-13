<script lang='js'>

export default {
    name: "UserSettings",
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
        <p>User Settings - WIP {{ this.guild }}</p>
    </div>
</template>

<style scoped></style>
