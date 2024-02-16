<script lang='js'>
export default {
    name: "StatisticsPage",
    props: {
        user: Object,
        guilds: Array
    },
    created() {
        if (!this.user) {
            return this.$router.push("/unauthorized");
        } else if (new Date(this.user.exp * 1000) < new Date()) return this.$router.push("/session-expired");
    }
};
</script>

<template>
    <div class="content">
        <div class="content-wrapper">
            <p>Statistics</p>
        </div>
    </div>
</template>

<style scoped></style>
