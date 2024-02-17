<script lang='js'>
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "LoginPage",
    data() {
        return {
            errorStatus: false,
            errorMessage: ""
        }
    },
    props: {
        user: Object,
        guilds: Array
    },
    emits: [
        "login",
        "popup"
    ],
    methods: {
        /**
         * Sign-in and load all data.
         * @param {Object} event DOM Click event.
         */
        login(event) {
            event.preventDefault();
            const username = document.getElementById("username-input").value;
            const password = document.getElementById("password-input").value;

            if (username.length === 0 || password.length === 0) {
                this.errorStatus = true;
                return this.errorMessage = "Please fill in your credentials.";
            }

            invoke("login", { username: username, password: password }).then((userData) => {
                try {
                    invoke("fetch_guild", { username: userData.operator_username }).then(async (rawGuildData) => {
                        const guildData = JSON.parse(rawGuildData);
                        this.$emit("login", userData, guildData);
                    }).catch((error) => {
                        console.error(error);
                        this.errorStatus = true;
                        this.errorMessage = "Something went wrong while retrieving your information. Please try again later.";
                    });
                } catch (error) {
                    console.error(error);
                    this.errorStatus = true;
                    this.errorMessage = "Something went wrong while processing your information. Please try again later.";
                }
            }).catch((error) => {
                if (error === "Not Found") {
                    this.errorStatus = true;
                    this.errorMessage = "This username does not exist.";
                } else if (error === "Unauthorized") {
                    this.errorStatus = true;
                    this.errorMessage = "Your username or password is incorrect.";
                } else {
                    this.errorStatus = true;
                    this.errorMessage = "Something went wrong while signing in. Please try again later.";
                }
            });
        }
    }
};
</script>

<template>
    <div class="content">
        <section class="login-card">
            <div class="header">
                <h2>Welcome back</h2>
                <h5 class="sub-header">Sign in to continue</h5>
            </div>
            <form>
                <div class="input-container">
                    <div class="input-wrapper">
                        <img src="../assets/images/gold.png" class="input-image">
                        <div class="input-content">
                            <i class="fa-solid fa-user faded-text"></i>
                            <input placeholder="Username" autocomplete="username" id="username-input" class="text-input"
                                type="text">
                        </div>
                    </div>
                    <div class="password-wrapper">
                        <div class="input-wrapper">
                            <img src="../assets/images/gold.png" class="input-image">
                            <div class="input-content">
                                <i class="fa-solid fa-key faded-text"></i>
                                <input placeholder="Password" autocomplete="current-password" id="password-input"
                                    class="text-input" type="password">
                            </div>
                        </div>
                        <router-link to="/"
                            @click="this.$emit('popup', 'warning', 'Password reset is still WIP.', 4000)">Forgot your
                            password?</router-link>
                    </div>
                </div>
                <section class="submit-container">
                    <div class="input-wrapper">
                        <img src="../assets/images/gold.png" class="input-image">
                        <button class="login-button" @click="this.login($event);">Login</button>
                    </div>
                    <router-link to="/" @click="this.$emit('popup', 'warning', 'Registration is still WIP.', 4000)">Don't
                        have an account yet?</router-link>
                </section>
            </form>
        </section>
        <section class="login-card error-card" v-if="this.errorStatus">
            <p class="error-message">{{ this.errorMessage }}</p>
        </section>
    </div>
</template>

<style scoped>
body {
    overflow: hidden;
}

.content {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 70px;
    padding: 0;
    gap: 20px;
}

.login-card {
    width: 270px;
    height: 375px;
    background-color: var(--fill);
    box-sizing: border-box;
    padding: 20px;
    border-radius: var(--border-radius-high);
    display: flex;
    flex-direction: column;
    gap: 50px;
}

.error-card {
    min-height: 50px;
    height: unset;
    gap: 0;
    align-items: center;
    justify-content: center;
}

.error-message {
    color: var(--danger);
    text-align: center;
}

.header {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.sub-header {
    color: var(--accent);
}

form {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 40px;
}

.input-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.input-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    border-radius: var(--border-radius-high);
    width: 190px;
    height: 40px;
}

.input-image {
    position: absolute;
    width: 190px;
    height: 40px;
    object-fit: cover;
    border-radius: var(--border-radius-high);
}

.input-content {
    position: absolute;
    z-index: 1;
    background-color: var(--fill);
    width: 187px;
    height: 37px;
    border-radius: 14px;
    display: flex;
    align-items: center;
    box-sizing: border-box;
    padding-left: 10px;
    gap: 5px;
}

.password-wrapper {
    display: flex;
    flex-direction: column;
}

.login-button {
    position: relative;
    z-index: 1;
    width: 190px;
    background-color: transparent;
    color: var(--main);
    font-size: medium;
    text-transform: uppercase;
    font-weight: 800;
}

input {
    width: 80%;
    color: var(--font);
    font-weight: bold;
}

.submit-container,
.password-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
}

.active {
    background-color: transparent;
    color: var(--font-mid);
    border-radius: 0;
    -moz-box-shadow: none;
    -webkit-box-shadow: none;
    box-shadow: none;
    font-size: small;
}
</style>
