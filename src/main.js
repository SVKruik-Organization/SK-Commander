import { createApp } from "vue";
import App from "./App.vue";
import "./assets/styles/main.css";
import "./assets/styles/home.css";
import router from "./router.js";
import guild from './assets/stores/guild.js'

createApp(App).use(router).use(guild).mount("#app");
