<script lang='js'>
export default {
    name: "SupportPage",
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
            <p>Support Page - {{ this.user.snowflake }}</p>
            <p v-if="this.user.edition === 'Enterprise'">Your current plan does not include dedicated support from our team.
                Upgrade or consult community support instead.</p>
        </div>
    </div>
</template>

<style scoped></style>