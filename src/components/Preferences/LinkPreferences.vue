<script lang='js'>
export default {
    name: "LinkPreferences",
    props: {
        user: Object,
        guilds: Array
    }
};
</script>

<template>
    <div class="content-container">
        <section class="settings-component">
            <p class="header bold text-shadow">Resources</p>
            <a href="https://platform.stefankruik.com/documentation/read/Doc/Community/Links" target="_blank"
                class="setting-item">
                <p class="link-text">Discord Community</p>
                <i class="fa-brands fa-discord link-icon"></i>
            </a>
            <a href="https://platform.stefankruik.com/documentation/read/Doc/Community/Support" target="_blank"
                class="setting-item">
                <p class="link-text">Support</p>
                <i class="fa-solid fa-envelope link-icon"></i>
            </a>
        </section>
        <section class="settings-component">
            <p class="header bold text-shadow">Source Code</p>
            <a href="https://github.com/SVKruik-Organization/SK-Commander" target="_blank" class="setting-item">
                <p class="link-text">SK Commander Repository</p>
                <i class="fa-solid fa-code-branch link-icon"></i>
            </a>
            <a href="https://github.com/SVKruik-Organization/SK-Bots" target="_blank" class="setting-item">
                <p class="link-text">SK Bots Repository</p>
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
