<script lang='js'>

export default {
    name: "StatisticsPage",
    props: {
        jwt: String,
        guilds: Array
    },
    mounted() {
        if (!this.jwt) return this.$router.push("/unauthorized");
    }
};
</script>

<template>
    <div class="content">
        <p>Statistics</p>
    </div>
</template>

<style scoped></style>
