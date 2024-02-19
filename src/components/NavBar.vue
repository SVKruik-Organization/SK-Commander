<script lang='js'>
export default {
    name: "Navbar",
    props: {
        user: Object
    },
    emits: [
        "logout"
    ],
    mounted() {
        // Close Dropdown
        document.body.addEventListener("click", (event) => {
            if (!event.target.classList.contains("dropdown-icon") && !event.target.classList.contains("profile-picture")) {
                this.closeDropdown();
            }
        });
    },
    methods: {
        /**
         * Open or close the dropdown.
         */
        toggleDropdown() {
            const caret = document.getElementsByClassName("dropdown-icon")[0];
            const menu = document.getElementsByClassName("dropdown-menu")[0];
            if (caret.classList.contains("rotated")) {
                this.closeDropdown();
            } else {
                if (caret) caret.classList.add("rotated");
                if (menu) menu.classList.add("visible");
            }
        },
        /**
         * Close the dropdown, regardless of current state.
         */
        closeDropdown() {
            const caret = document.getElementsByClassName("dropdown-icon")[0];
            const menu = document.getElementsByClassName("dropdown-menu")[0];
            if (caret) caret.classList.remove("rotated");
            if (menu) menu.classList.remove("visible");
        },
        /**
         * Route the user to another page.
         * @param {string} targetRoute The target location.
         */
        link(targetRoute) {
            this.closeDropdown();
            this.$router.push(targetRoute);
        }
    }
};
</script>

<template>
    <nav class="shadow">
        <section class="links-container">
            <div class="link-item">
                <router-link class="link" to="/home">Home</router-link>
                <span class="nav-indicator"></span>
            </div>
            <div class="link-item">
                <router-link class="link" to="/settings/general">Settings</router-link>
                <span class="nav-indicator"></span>
            </div>
            <div class="link-item">
                <router-link class="link" to="/statistics">Statistics</router-link>
                <span class="nav-indicator"></span>
            </div>
            <div class="link-item">
                <router-link class="link" to="/operators">Operators</router-link>
                <span class="nav-indicator"></span>
            </div>
        </section>
        <section class="personal-container">
            <span class="profile-picture shadow" :style="`background-image: url(${this.user ? this.user.avatar : null});`"
                @click="this.toggleDropdown();"></span>
            <i class="fa-solid fa-caret-down dropdown-icon" @click="this.toggleDropdown();"></i>
        </section>
    </nav>
    <div class="dropdown-menu">
        <div class="dropdown-item">
            <div class="information-wrapper">
                <h5 class="username">{{ this.user ? this.user.operator_username : 'Not Logged In' }}</h5>
                <h6 class="tag">{{ this.user ? `@${this.user.user_username}` : 'Not Logged In' }}</h6>
            </div>
        </div>
        <span class="splitter"></span>
        <div class="dropdown-item">
            <router-link to="/home" class="dropdown-link">Home</router-link>
            <router-link to="/preferences/account" class="dropdown-link">Preferences</router-link>
            <router-link to="/support" class="dropdown-link">Support</router-link>
        </div>
        <span class="splitter"></span>
        <div class="dropdown-item">
            <router-link to="/plans" v-if="this.user.edition === 'Basic'" class="dropdown-link-wrapper dropdown-link">
                <h6 class="sign-out-text pointer">Upgrade Plan</h6>
                <i class="fa-solid fa-arrow-up upgrade-icon pointer"></i>
            </router-link>
            <div @click="this.$emit('logout');" class="dropdown-link-wrapper dropdown-link">
                <h6 class="sign-out-text pointer">Sign Out</h6>
                <i class="fa-solid fa-arrow-right-from-bracket sign-out-icon pointer"></i>
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
    background-size: 36px;
    background-color: var(--accent);
    background-repeat: no-repeat;
    background-position: center;
    border-radius: 50%;
    cursor: pointer;
}

.dropdown-icon {
    color: var(--font-mid);
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
    right: 10px;
    top: 55px;
    background-color: var(--fill-light);
    border-radius: var(--border-radius-low);
    border: 1px solid var(--font-mid);
    user-select: none;
    z-index: 1;
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

.dropdown-link p {
    font-size: 12px;
    font-weight: bold;
}

.upgrade-icon {
    color: var(--accent);
    margin-right: 2px;
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

.active {
    background-color: transparent;
    color: var(--font-main);
    border-radius: 0;
    -moz-box-shadow: none;
    box-shadow: none;
    -webkit-box-shadow: none;
    border-radius: var(--border-radius-low);
}
</style>
