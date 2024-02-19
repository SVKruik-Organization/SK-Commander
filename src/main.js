import { createApp } from "vue";
import App from "./App.vue";
import "./assets/styles/main.css";
import "./assets/styles/settings.css";
import "./assets/styles/menu.css";
import "./assets/styles/preferences.css";
import router from "./router.js";
import store from "./assets/stores/index.js";

createApp(App).use(router).use(store).mount("#app");
