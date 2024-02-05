<script lang='js'>
export default {
    name: "Navbar",
    methods: {
        toggleDropdown() {
            const caret = document.getElementsByClassName("dropdown-icon")[0];
            const menu = document.getElementsByClassName("dropdown-menu")[0];
            if (caret.classList.contains("rotated")) {
                caret.classList.remove("rotated");
                menu.classList.remove("visible");
            } else {
                caret.classList.add("rotated");
                menu.classList.add("visible");
            }
        },
        link(targetRoute) {
            caret.classList.remove("rotated");
            menu.classList.remove("visible");
            this.$router.push(targetRoute);
        },
        signOut() {
            this.$router.push("/");
        }
    }
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
            <div class="link-item">
                <router-link class="link" to="/greet">Greet</router-link>
                <span class="nav-indicator"></span>
            </div>
        </section>
        <section class="personal-container">
            <img class="profile-picture shadow" @click="this.toggleDropdown()">
            <i class="fa-solid fa-caret-down dropdown-icon" @click="this.toggleDropdown()"></i>
        </section>
    </nav>
    <div class="dropdown-menu" v-if="this.$route.path !== '/'">
        <div class="dropdown-item">
            <div class="information-wrapper">
                <h5 class="username">Stefan Kruik</h5>
                <h6 class="tag">@complex9078</h6>
            </div>
        </div>
        <span class="splitter"></span>
        <div class="dropdown-item">
            <router-link to="/preferences" class="dropdown-link">Preferences</router-link>
            <router-link to="/support" class="dropdown-link">Support</router-link>
        </div>
        <span class="splitter"></span>
        <div class="dropdown-item">
            <div @click="this.signOut()" class="dropdown-link-wrapper dropdown-link">
                <h6 class="sign-out-text">Sign Out</h6>
                <i class="fa-solid fa-arrow-right-from-bracket sign-out-icon"></i>
            </div>
        </div>
    </div>
</template>

<style scoped>
nav {
    background-color: var(--fill);
    height: 50px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    user-select: none;
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

.active+span {
    background-color: var(--accent);
}

.personal-container {
    display: flex;
    align-items: center;
    gap: 10px;
    height: 50px;
    margin-right: 20px;
    justify-content: space-between;
}

.profile-picture {
    width: 35px;
    height: 35px;
    object-fit: cover;
    background-color: var(--accent);
    border-radius: 50%;
    cursor: pointer;
}

.dropdown-icon {
    color: var(--font-light);
    cursor: pointer;
    rotate: 0deg;
}

.rotated {
    rotate: 180deg;
}

.dropdown-menu {
    display: none;
    flex-direction: column;
    position: absolute;
    width: 140px;
    height: fit-content;
    right: 5px;
    top: 55px;
    background-color: var(--fill-light);
    border-radius: var(--border-radius-low);
    border: 1px solid var(--font-light);
    user-select: none;
}

.dropdown-item {
    box-sizing: border-box;
    padding: 5px;
    display: flex;
    flex-direction: column;
}

.tag {
    font-weight: normal;
}

.splitter {
    width: 140px;
    height: 1px;
}

.dropdown-link {
    font-size: 12px;
    box-sizing: border-box;
    padding: 2px 5px;
    border-radius: var(--border-radius-low);
}

.visible {
    display: flex;
}

a:hover,
.dropdown-link-wrapper:hover {
    background-color: var(--fill);
}

.dropdown-link-wrapper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
}

.sign-out-icon {
    color: var(--danger);
}

.sign-out-text {
    font-size: 12px;
}

.information-wrapper {
    box-sizing: border-box;
    padding: 0 5px;
}
</style>
