<script lang='js'>

export default {
    name: "SessionExpiredPage",
    mounted() {
        this.$emit("logout");
    }
};
</script>

<template>
    <div class="content">
        <h1>401</h1>
        <h4>Your session has expired. Please sign in again.</h4>
        <router-link to="/">Sign In</router-link>
    </div>
</template>

<style scoped>
.content {
    display: flex;
    flex-direction: column;
    gap: 20px;
    align-items: center;
    justify-content: center;
}
</style>
