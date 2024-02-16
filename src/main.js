import { createApp } from "vue";
import App from "./App.vue";
import "./assets/styles/main.css";
import "./assets/styles/home.css";
import router from "./router.js";
import store from "./assets/stores/index.js";

createApp(App).use(router).use(store).mount("#app");
