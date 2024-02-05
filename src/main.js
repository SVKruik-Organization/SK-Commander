import { createApp } from "vue";
import App from "./App.vue";
import "./assets/styles/main.css";
import "./assets/styles/home.css";
import router from "./router.js";

createApp(App).use(router).mount("#app");
