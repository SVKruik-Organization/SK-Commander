<script lang='js'>
export default {
    name: "LinkSettings",
    props: {
        user: Object,
        guilds: Array
    }
};
</script>

<template>
    <div class="content-container">
        <section class="settings-component">
            <p class="header text-shadow">Resources</p>
            <a href="https://github.com/SVKruik" target="_blank" class="setting-item">
                <p class="link-text">Discord Community</p>
                <i class="fa-brands fa-discord link-icon"></i>
            </a>
            <a href="mailto:stefan.kruik@stefankruik.com?subject=Bot%20Commander%20Support&body=Use%20this%20email%20to%20contact%20support%20or%20for%20questions.%20When%20you%20need%20support%20with%20your%20account%2C%20please%20include%20your%20service%20tag%20%26%20username."
                target="_blank" class="setting-item">
                <p class="link-text">Email Support</p>
                <i class="fa-solid fa-envelope link-icon"></i>
            </a>
        </section>
        <section class="settings-component">
            <p class="header text-shadow">Source Code</p>
            <a href="https://github.com/SVKruik/bot-config-ui" target="_blank" class="setting-item">
                <p class="link-text">Bot Commander Repository</p>
                <i class="fa-solid fa-code-branch link-icon"></i>
            </a>
            <a href="https://github.com/SVKruik/Discord-Bots-v2" target="_blank" class="setting-item">
                <p class="link-text">Discord Bots Repository</p>
                <i class="fa-solid fa-code-branch link-icon"></i>
            </a>
            <a href="https://github.com/SVKruik" target="_blank" class="setting-item">
                <p class="link-text">Author</p>
                <i class="fa-brands fa-github link-icon"></i>
            </a>
        </section>
    </div>
</template>

<style scoped>
.content-container {
    box-sizing: border-box;
    padding-right: 20px;
}

.link-icon {
    color: var(--font-mid);
}
</style>
