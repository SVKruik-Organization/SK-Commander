<script lang='js'>

export default {
    name: "EconomySettings",
    props: {
        guild: Object
    },
    methods: {
        save(inputId, iconId) {
            const input = document.getElementById(inputId);
            if (!input) return;
            const icon = document.getElementById(iconId);
            if (!icon) return;
            icon.classList.remove("visible");

            // TODO - Database Update
        },
        change(iconId) {
            const icon = document.getElementById(iconId);
            if (!icon) return;
            icon.classList.add("visible");
        }
    }
};
</script>

<template>
    <div class="content-container">
        <section class="settings-item">
            <p class="header text-shadow">Prices</p>
            <div class="input-container">
                <p class="input-label text-shadow">XP +15% 24H</p>
                <div class="input-wrapper">
                    <input type="number" class="shadow input" id="priceXp15" @change="this.change('save-priceXp15')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-priceXp15"
                        @click="this.save('priceXp15', 'save-priceXp15')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">XP +50% 24H</p>
                <div class="input-wrapper">
                    <input type="number" class="shadow input" id="priceXp50" @change="this.change('save-priceXp50')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-priceXp50"
                        @click="this.save('priceXp50', 'save-priceXp50')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Cosmetic Role</p>
                <div class="input-wrapper">
                    <input type="number" class="shadow input" id="cosmeticRole" @change="this.change('save-cosmeticRole')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-cosmeticRole"
                        @click="this.save('cosmeticRole', 'save-cosmeticRole')"></i>
                </div>
            </div>
        </section>
        <section class="settings-item">
            <p class="header text-shadow">Rewards</p>
            <div class="input-container">
                <p class="input-label text-shadow">Level Up</p>
                <div class="input-wrapper">
                    <input type="number" class="shadow input" id="levelUp" @change="this.change('save-levelUp')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-levelUp"
                        @click="this.save('levelUp', 'save-levelUp')"></i>
                </div>
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Jackpot</p>
                <div class="input-wrapper">
                    <input type="number" class="shadow input" id="jackpot" @change="this.change('save-jackpot')">
                    <i class="fa-solid fa-floppy-disk save-icon" id="save-jackpot"
                        @click="this.save('jackpot', 'save-jackpot')"></i>
                </div>
            </div>
        </section>
    </div>
</template>

<style scoped></style>
