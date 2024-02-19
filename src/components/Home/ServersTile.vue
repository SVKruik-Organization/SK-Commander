<script lang='js'>
export default {
    name: "ServersTile",
    props: {
        user: Object,
        guilds: Array
    }
};
</script>

<template>
    <div class="tile tile-small shadow">
        <p class="tile-title bold">Servers</p>
        <ul class="server-container">
            <li class="server-item" v-for="guild in this.guilds">
            </li>
        </ul>
    </div>
</template>

<style scoped></style>
