<script lang='js'>
export default {
    name: "SessionExpiredPage",
    emits: [
        "logout"
    ]
};
</script>

<template>
    <div class="content">
        <div class="content-wrapper error-page">
            <h1>401</h1>
            <h4>Your session has expired. Please sign in again.</h4>
            <button @click="this.$emit('logout');">Sign In</button>
        </div>
    </div>
</template>

<style scoped>
.content {
    overflow: hidden;
}
</style>
