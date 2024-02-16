<script lang='js'>
export default {
    name: "StatisticsPage",
    props: {
        user: Object,
        guilds: Array
    },
    mounted() {
        if (!this.user) return this.$router.push("/unauthorized");
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
