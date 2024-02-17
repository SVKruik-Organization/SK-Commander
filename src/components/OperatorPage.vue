<script lang='js'>
export default {
    name: "OperatorPage",
    props: {
        user: Object
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
            <p>Operator Page - {{ this.user.snowflake }}</p>
        </div>
    </div>
</template>

<style scoped></style>