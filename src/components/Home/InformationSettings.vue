<script lang='js'>

export default {
    name: "InformationSettings",
    props: {
        guild: Object
    },
};
</script>

<template>
    <div class="content-container">
        <section class="settings-item">
            <p class="header text-shadow">Constants</p>
            <div class="input-container">
                <p class="input-label text-shadow">Server ID</p>
                <input type="text" disabled id="constantSnowflake" class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Registerer ID</p>
                <input type="text" disabled id="constantRegisterSnowflake" class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Name</p>
                <input type="text" disabled id="constantName" class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Locale</p>
                <input type="text" disabled id="constantLocale" class="shadow input">
            </div>
            <div class="input-container">
                <p class="input-label text-shadow">Status</p>
                <div class="input-wrapper">
                    <div class="select-wrapper shadow">
                        <select id="constantStatus" disabled>
                            <option>Enabled</option>
                            <option>Disabled</option>
                        </select>
                        <i class="fa-solid fa-caret-down select-icon"></i>
                    </div>
                </div>
            </div>
        </section>
    </div>
</template>

<style scoped>
.input {
    cursor: text;
    margin-right: 30px;
}

.checkbox,
.input-wrapper {
    margin-right: 30px;
}

.select-wrapper {
    width: 160px;
    height: 25px;
    border-radius: var(--border-radius-low);
    background-color: var(--fill-mid);
    border: 2px solid #FFFFFF10;
}

select {
    width: 150px;
    cursor: default;
}
</style>
