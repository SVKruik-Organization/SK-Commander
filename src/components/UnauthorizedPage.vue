<script lang='js'>

export default {
    name: "UnauthorizedPage",
    mounted() {
        this.$emit("logout");
    }
};
</script>

<template>
    <div class="content">
        <h1>401</h1>
        <h4>Uh oh! The page you tried to visit requires you to be signed in!</h4>
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
