<script lang='js'>
export default {
    name: "SessionExpiredPage",
    props: {
        pendingPopups: Array
    },
    emits: [
        "logout"
    ],
    methods: {
        /**
         * Close a active popup.
         * @param {string} id The ID of the popup element to delete.
         */
        closePopup(id) {
            const elements = document.getElementsByClassName(`parent-${id}`);
            if (elements.length === 0) return;
            elements[0].classList.remove("visible");
            setTimeout(() => {
                if (elements[0]) elements[0].remove();
            }, 250);
        },
        /**
         * Softly show the notification, instead of instant display.
         * @param {string} id The ID of the popup element to fade-in.
         */
        fadeIn(id) {
            const elements = document.getElementsByClassName(`parent-${id}`);
            if (elements.length === 0) return;
            elements[0].classList.add("visible");
        }
    }
};
</script>

<template>
    <section class="popup-container">
        <div @click="this.closePopup($event.target.id)" class="popup shadow" v-for="popup in this.pendingPopups"
            :class="`parent-${popup.id}`" :id="popup.id">
            <div class="left pointer" :id="popup.id">
                <span class="color-indicator pointer" :id="popup.id" :style="`background-color: ${popup.color}`"></span>
                <p class="message pointer" :id="popup.id">{{ popup.message }}</p>
            </div>
            <div class="right pointer" :id="popup.id">
                <button class="close-popup-button" :id="popup.id">
                    <i class="fa-solid fa-xmark close-icon" :id="popup.id"></i>
                </button>
            </div>
        </div>
    </section>
</template>

<style scoped>
.popup-container {
    width: 400px;
    position: absolute;
    left: 0;
    right: 0;
    bottom: 15px;
    margin-left: auto;
    margin-right: auto;
    z-index: 1;
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.popup {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 30px;
    background-color: var(--fill-light);
    border: 2px solid var(--font-light);
    border-radius: var(--border-radius-low);
    cursor: pointer;
    opacity: 0;
}

.visible {
    opacity: 1;
}

.left,
.right {
    height: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
}

.color-indicator {
    background-color: cornflowerblue;
    height: 100%;
    width: 5px;
    border-top-left-radius: var(--border-radius-low);
    border-bottom-left-radius: var(--border-radius-low);
}

.close-popup-button {
    background-color: transparent;
    height: 100%;
    width: 30px;
    opacity: 0;
    display: flex;
    align-items: center;
    justify-content: center;
}

.popup:hover .close-popup-button {
    opacity: 1;
}

.close-icon {
    color: var(--font-mid);
    cursor: pointer;
}
</style>
