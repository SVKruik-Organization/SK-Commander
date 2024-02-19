<script lang='js'>
export default {
    name: "InteractionTile",
    props: {
        user: Object,
        guilds: Array
    }
};
</script>

<template>
    <div class="tile tile-mid shadow">
        <p class="tile-title bold">Purchases</p>
        <ul class="purchase-container">
            <li class="purchase-item">
                <p class="purchase-item-time">13:59</p>
                <p class="purchase-item-quantity">[1x]</p>
                <p class="purchase-item-description ellipsis">XP +50% 24 H</p>
            </li>
        </ul>
    </div>
</template>

<style scoped>
.purchase-item {
    display: flex;
    align-items: center;
    gap: 2px;
}

.purchase-item p {
    font-size: var(--font-size-mid);
}
</style>
