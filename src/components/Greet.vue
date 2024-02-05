<script lang="js">
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "GreetPage",
    data() {
        return {
            message: ""
        }
    },
    methods: {
        async greet() {
            const input = document.getElementById("greet-input");
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            this.message = await invoke("greet", { name: input.value });
        }
    }
};
</script>

<template>
    <div class="content">
        <form class="row" @submit.prevent="this.greet()">
            <input id="greet-input" v-model="name" style="width: 100px;" placeholder="Enter a name..." />
            <button type="submit">Greet</button>
        </form>

        <p>{{ message }}</p>
    </div>
</template>
