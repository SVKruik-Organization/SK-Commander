<script lang='js'>

export default {
    name: "HomePage",
    data() {
        return {
            activeGuild: {}
        }
    }
};
</script>

<template>
    <div class="content main-content">
        <section class="select-container">
            <div class="select-wrapper shadow">
                <select>
                    <option>Guild Name A</option>
                    <option>Guild Name B</option>
                    <option>Guild Name C</option>
                </select>
                <i class="fa-solid fa-caret-down select-icon"></i>
            </div>
        </section>
        <section class="settings-container">
            <div class="menu-list">
                <router-link class="menu-item text-shadow" to="/home/general">General</router-link>
                <span class="splitter shadow"></span>
                <router-link class="menu-item text-shadow" to="/home/economy">Economy</router-link>
                <span class="splitter shadow"></span>
                <router-link class="menu-item text-shadow" to="/home/level">Level</router-link>
                <span class="splitter shadow"></span>
                <router-link class="menu-item text-shadow" to="/home/users">Users</router-link>
                <span class="splitter shadow"></span>
                <router-link class="menu-item text-shadow" to="/home/information">Information</router-link>
            </div>
            <span class="splitter-accent"></span>
            <router-view :guild="this.activeGuild"></router-view>
        </section>
        <router-link to="/">Login</router-link>
    </div>
</template>

<style scoped>
.main-content {
    margin: auto;
    margin-top: 10px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    width: 560px;
}

.select-container {
    display: flex;
    gap: 10px;
}

.settings-container {
    display: flex;
    justify-content: space-between;
    background-color: var(--fill);
    border-radius: var(--border-radius-high);
    height: 440px;
    box-sizing: border-box;
    padding: 35px 0 35px 35px;
}

.menu-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: 120px;
}

.menu-item {
    height: 32px;
    width: 120px;
    font-weight: bold;
    display: flex;
    align-items: center;
    box-sizing: border-box;
    padding-left: 10px;
}

.active {
    background-color: var(--fill-mid);
    color: var(--accent);
    border-radius: var(--border-radius-low);
    -moz-box-shadow: 0 4px 4px rgba(0, 0, 0, 0.1);
    -webkit-box-shadow: 0 4px 4px rgba(0, 0, 0, 0.1);
    box-shadow: 0 4px 4px rgba(0, 0, 0, 0.1);
}

.splitter {
    width: 120px;
    height: 2px;
}

.splitter-accent {
    height: 360px;
    width: 2px;
}
</style>
