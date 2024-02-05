<script lang='js'>
export default {
    name: "LoginPage",
    methods: {
        login(event) {
            event.preventDefault();
            this.$router.push("/home/general");
        }
    }
};
</script>

<template>
    <div class="content">
        <section class="login-card">
            <div class="header">
                <h1>Welcome back</h1>
                <h5 class="sub-header">Sign in to continue</h5>
            </div>
            <form>
                <div class="input-container">
                    <div class="input-wrapper">
                        <img src="../assets/images/gold.png" class="input-image">
                        <div class="input-content">
                            <i class="fa-solid fa-user faded-text"></i>
                            <input placeholder="Username" class="text-input" type="text">
                        </div>
                    </div>
                    <div class="password-wrapper">
                        <div class="input-wrapper">
                            <img src="../assets/images/gold.png" class="input-image">
                            <div class="input-content">
                                <i class="fa-solid fa-key faded-text"></i>
                                <input placeholder="Password" class="text-input" type="password">
                            </div>
                        </div>
                        <a href="https://github.com/SVKruik/Discord-Bots-v2" target="_blank" class="faded-text small">
                        Forgot password?</a>
                    </div>
                </div>
                <section class="submit-container">
                    <div class="input-wrapper">
                        <img src="../assets/images/gold.png" class="input-image">
                        <button class="login-button" @click="this.login($event)">Login</button>
                    </div>
                    <a href="https://github.com/SVKruik/Discord-Bots-v2" target="_blank" class="faded-text small">
                        Don't have an account yet?</a>
                </section>
            </form>
        </section>
    </div>
</template>

<style scoped>
.content {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
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
</style>
