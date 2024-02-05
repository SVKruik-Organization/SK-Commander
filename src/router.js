import { createRouter, createWebHistory } from "vue-router";
import StatisticsPage from "./components/StatisticsPage.vue";
import HomePage from "./components/HomePage.vue";
import LoginPage from "./components/LoginPage.vue";
import GeneralSettings from "./components/Home/GeneralSettings.vue";
import EconomySettings from "./components/Home/EconomySettings.vue";
import UserSettings from "./components/Home/UserSettings.vue";

const router = createRouter({
    history: createWebHistory(),
    linkActiveClass: "active",
    routes: [
        { path: "/", component: LoginPage },
        {
            path: "/home", component: HomePage, children: [
                { path: "", redirect: "/home/general" },
                { path: "general", component: GeneralSettings },
                { path: "economy", component: EconomySettings },
                { path: "users", component: UserSettings },
            ]
        },
        { path: "/statistics", component: StatisticsPage },
        { path: "/:pathMatch(.*)", redirect: "/" }
    ]
});

export default router;
