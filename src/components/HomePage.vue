<script lang='js'>
import InteractionTile from './Home/InteractionTile.vue';
import LogsTile from './Home/LogsTile.vue';
import OperatorsTile from './Home/OperatorsTile.vue';
import ServersTile from './Home/ServersTile.vue';
import PurchasesTile from './Home/PurchasesTile.vue';

export default {
    name: "HomePage",
    props: {
        user: Object,
        guilds: Array,
    },
    created() {
        if (!this.user) {
            return this.$router.push("/unauthorized");
        } else if (new Date(this.user.exp * 1000) < new Date()) return this.$router.push("/session-expired");
    },
    components: { InteractionTile, LogsTile, OperatorsTile, ServersTile, PurchasesTile }
};
</script>

<template>
    <div class="content">
        <div class="content-wrapper">
            <h3 class="text-shadow">Welcome, {{ this.user.operator_username }}</h3>
            <div class="tile-container-wrapper">
                <section class="tile-container">
                    <InteractionTile :user="this.user" :guilds="this.guilds"></InteractionTile>
                    <LogsTile :user="this.user" :guilds="this.guilds"></LogsTile>
                </section>
                <section class="tile-container">
                    <OperatorsTile :user="this.user" :guilds="this.guilds"></OperatorsTile>
                    <ServersTile :user="this.user" :guilds="this.guilds"></ServersTile>
                    <PurchasesTile :user="this.user" :guilds="this.guilds"></PurchasesTile>
                </section>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Tile Containers */
.tile-container-wrapper {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.tile-container {
    display: flex;
    gap: 10px;
}
</style>