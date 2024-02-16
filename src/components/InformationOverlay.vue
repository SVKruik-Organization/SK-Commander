<script lang='js'>
export default {
    name: "InformationOverlay",
    emits: [
        "toggleInformationOverlay"
    ]
};
</script>

<template>
    <div class="overlay-container">
        <section class="overlay-item">
            <h3>Information</h3>
            <article>
                <p>On this page you can modify the settings and variables of your server.</p>
                <p>For some inputs (channels, roles), the ID is required. To grab the ID of a Discord object, enable
                    Developer Mode in the Advanced settings tab. Then, right-click on the channel/role and will see an
                    option to copy the ID.</p>
                <p>When you have updated a value, you will see an icon appear. When you click on this, your new value will
                    be saved.</p>
                <p>When tweaking Economy or Level settings, make sure to keep everything consistent, reachable and viable.
                </p>
            </article>
        </section>
        <div class="overlay-background" @click.self="this.$emit('toggleInformationOverlay');"></div>
    </div>
</template>

<style scoped>
.overlay-container {
    position: absolute;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.overlay-item {
    position: absolute;
    background-color: var(--fill);
    box-sizing: border-box;
    padding: 15px;
    z-index: 3;
    width: 500px;
    min-height: 300px;
    border-radius: var(--border-radius-high);
    display: flex;
    flex-direction: column;
    gap: 20px;
}

article {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

h3 {
    color: var(--accent);
}

.overlay-background {
    position: absolute;
    z-index: 2;
    background-color: #00000050;
    backdrop-filter: blur(2px);
    width: 100%;
    height: 100%;
}</style>
