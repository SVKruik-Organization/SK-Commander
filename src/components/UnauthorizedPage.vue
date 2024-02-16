<script lang='js'>

export default {
    name: "UnauthorizedPage",
    emits: [
        "logout"
    ]
};
</script>

<template>
    <div class="content">
        <div class="content-wrapper error-page">
            <h1>401</h1>
            <h4>Uh oh! The page you tried to visit requires you to be signed in!</h4>
            <button @click="this.$emit('logout');">Sign In</button>
        </div>
    </div>
</template>

<style scoped>
.content {
    overflow: hidden;
}
</style>
