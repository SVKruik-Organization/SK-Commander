<script lang='js'>

export default {
    name: "HomePage"
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
                <router-link class="menu-item text-shadow" to="/home/users">Users</router-link>
            </div>
            <span class="splitter-accent"></span>
            <router-view></router-view>
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

.select-wrapper {
    position: relative;
    display: flex;
    width: 200px;
    align-items: center;
    border-radius: var(--border-radius-mid);
    border: 1px solid var(--font-light);
    background-color: var(--fill);
    justify-content: flex-end;
    height: 30px;
    box-sizing: border-box;
}

select {
    width: 190px;
    position: absolute;
    z-index: 1;
}

.select-icon {
    color: var(--font-light);
    position: absolute;
    width: 20px;
}

.settings-container {
    display: flex;
    gap: 20px;
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
    background-color: var(--fill-light);
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
