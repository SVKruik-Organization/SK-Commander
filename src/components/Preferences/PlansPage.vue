<script lang='js'>
export default {
    name: "PlansPage",
    props: {
        user: Object
    },
    created() {
        if (!this.user) {
            return this.$router.push("/unauthorized");
        } else if (new Date(this.user.exp * 1000) < new Date()) return this.$router.push("/session-expired");
    },
    mounted() {
        // Setup Active Button
        const activePlanButton = document.getElementById(`plan-${this.user.edition.toLowerCase()}`);
        activePlanButton.classList.add("active-plan-button");
        activePlanButton.disabled = true;
        activePlanButton.innerHTML = "Current Plan";
    },
    methods: {
        upgrade(event) {
            const targetPlan = event.target.id.split("-")[1];
            console.log(targetPlan);
        }
    }
};
</script>

<template>
    <div class="content">
        <div class="content-wrapper">
            <section class="plan-card-container">
                <div class="plan-card shadow">
                    <h3>Basic</h3>
                    <span class="splitter shadow"></span>
                    <div class="price-container">
                        <small>€</small>
                        <h1>0</h1>
                        <small class="faded-text"></small>
                    </div>
                    <span class="splitter shadow"></span>
                    <small class="smaller bold">Best for Operators just starting out managing their friend groups.</small>
                    <small class="smaller faded-text perk-list-description"></small>
                    <ul class="perks-list">
                        <li class="perk-item">1 Operator</li>
                        <li class="perk-item">Up to 3 Servers</li>
                        <li class="perk-item">Community Support</li>
                    </ul>
                    <button class="select-button bold" id="plan-basic" @click="this.upgrade($event)">Get Started</button>
                </div>
                <div class="plan-card shadow">
                    <h3>Professional</h3>
                    <span class="splitter shadow"></span>
                    <div class="price-container">
                        <small>€</small>
                        <h1>1</h1>
                        <small class="faded-text">/ month</small>
                    </div>
                    <span class="splitter shadow"></span>
                    <small class="smaller bold">Best for small teams taking it up a notch.</small>
                    <small class="smaller faded-text perk-list-description">Everything in Basic, and</small>
                    <ul class="perks-list">
                        <li class="perk-item">Up to 3 Operators</li>
                        <li class="perk-item">Up to 5 Servers</li>
                        <li class="perk-item">Statistics & Logging</li>
                        <li class="perk-item">Dedicated Support</li>
                    </ul>
                    <button class="select-button bold" id="plan-professional" @click="this.upgrade($event)">Get Started</button>
                </div>
                <div class="plan-card shadow">
                    <h3>Enterprise</h3>
                    <span class="splitter shadow"></span>
                    <div class="price-container">
                        <small>€</small>
                        <h1>3</h1>
                        <small class="faded-text">/ month</small>
                    </div>
                    <span class="splitter shadow"></span>
                    <small class="smaller bold">Best for large teams managing their communities.</small>
                    <small class="smaller faded-text perk-list-description">Everything in Professional, and</small>
                    <ul class="perks-list">
                        <li class="perk-item">Up to 10 Operators</li>
                        <li class="perk-item">Up to 25 Servers</li>
                        <li class="perk-item">Advanced Statistics & Logging</li>
                        <li class="perk-item">Priority Dedicated Support</li>
                        <li class="perk-item">Early Access & Bèta Program</li>
                    </ul>
                    <button class="select-button bold" id="plan-enterprise" @click="this.upgrade($event)">Get Started</button>
                </div>
            </section>
            <section class="footer">
                <div class="flex">
                    <p>Need a custom solution?</p>
                    <p>Don't hesitate to reach out.</p>
                </div>
                <a href="mailto:stefan.kruik@stefankruik.com?subject=Bot%20Commander%20Support&body=Use%20this%20email%20to%20contact%20support%20or%20for%20questions.%20When%20you%20need%20support%20with%20your%20account%2C%20please%20include%20your%20service%20tag%20%26%20username."
                    class="flex" target="_blank">
                    <p>Contact sales</p>
                    <i class="fa-solid fa-envelope link-icon faded-text"></i>
                </a>
            </section>
        </div>
    </div>
</template>

<style scoped>
.content {
    overflow: hidden;
}

.content-wrapper {
    justify-content: center;
    width: fit-content;
    height: 100%;
    margin-top: -30px;
}

.plan-card-container {
    display: flex;
    align-items: center;
    gap: 20px;
}

.plan-card {
    display: flex;
    flex-direction: column;
    gap: 10px;
    box-sizing: border-box;
    padding: 10px;
    background-color: var(--fill);
    width: 210px;
    height: 400px;
    border-radius: var(--border-radius-mid);
}

.splitter {
    height: 1px;
    width: 100%;
}

.price-container {
    display: flex;
    gap: 2px;
    margin-bottom: -5px;
}

.price-container>*:last-child {
    margin-left: 10px;
}

.perk-list-description {
    margin-top: 10px;
    height: 16px;
}

.perks-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.perk-item {
    font-size: var(--font-size-mid);
}

.plan-card>*:last-child {
    margin-top: auto;
}

.select-button {
    background-color: var(--accent);
    border-radius: var(--border-radius-mid);
    height: 25px;
    color: var(--main);
    font-size: var(--font-size-small);
}

.active-plan-button {
    opacity: 0.4;
}

.footer {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    gap: 5px;
    width: fit-content;
    margin: 0 auto;
}

.link-icon {
    line-height: 10px;
}
</style>