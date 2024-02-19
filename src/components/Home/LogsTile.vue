<script lang='js'>
export default {
    name: "LogsTile",
    props: {
        user: Object,
        guilds: Array
    }
};
</script>

<template>
    <div class="tile tile-large shadow">
        <p class="tile-title bold">Logs</p>
        <ul class="logs-container">
            <li class="log-item">
                <p class="log-item-time">13:59</p>
                <p class="log-item-type">[INFO]</p>
                <p class="log-item-text ellipsis">Due dates already up-to-date (none pending).</p>
            </li>
        </ul>
    </div>
</template>

<style scoped>
.logs-container {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.log-item,
.purchase-item {
    display: flex;
    align-items: center;
    gap: 2px;
}

.log-item p,
.purchase-item p {
    font-size: var(--font-size-mid);
}
</style>
