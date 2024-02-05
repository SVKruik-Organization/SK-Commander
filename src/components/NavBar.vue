<script lang='js'>
export default {
    name: "Navbar"
};
</script>

<template>
    <nav class="shadow" v-if="this.$route.path !== '/'">
        <section class="links-container">
            <div class="link-item">
                <router-link class="link" to="/home">Home</router-link>
                <span class="nav-indicator"></span>
            </div>
            <div class="link-item">
                <router-link class="link" to="/statistics">Statistics</router-link>
                <span class="nav-indicator"></span>
            </div>
        </section>
    </nav>
</template>

<style scoped>
nav {
    background-color: var(--fill);
    height: 50px;
    display: flex;
    align-items: center;
}

.links-container {
    display: flex;
    box-sizing: border-box;
    padding-left: 20px;
    height: 50px;
    justify-content: space-between;
}

.link-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    width: 110px;
    margin-top: 15px;
}

.link {
    font-weight: 700;
}

.nav-indicator {
    height: 2px;
    width: 90px;
    border-top-left-radius: 15px;
    border-top-right-radius: 15px;
    background-color: var(--fill);
}

.active + span {
    background-color: var(--accent);
}
</style>
