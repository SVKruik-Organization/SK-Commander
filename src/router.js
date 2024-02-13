import { createRouter, createWebHistory } from "vue-router";
import StatisticsPage from "./components/StatisticsPage.vue";
import HomePage from "./components/HomePage.vue";
import LoginPage from "./components/LoginPage.vue";
import GeneralSettings from "./components/Home/GeneralSettings.vue";
import EconomySettings from "./components/Home/EconomySettings.vue";
import LevelSettings from "./components/Home/LevelSettings.vue";
import UserSettings from "./components/Home/UserSettings.vue";
import InformationSettings from "./components/Home/InformationSettings.vue";
import UnauthorizedPage from "./components/UnauthorizedPage.vue";
import SessionExpiredPage from "./components/SessionExpiredPage.vue";

const router = createRouter({
    history: createWebHistory(),
    linkActiveClass: "active",
    routes: [
        { path: "/", component: LoginPage, props: true },
        {
            path: "/home", component: HomePage, props: true, children: [
                { path: "", redirect: "/home/general" },
                { path: "general", component: GeneralSettings, props: true },
                { path: "economy", component: EconomySettings, props: true },
                { path: "level", component: LevelSettings, props: true },
                { path: "users", component: UserSettings, props: true },
                { path: "information", component: InformationSettings, props: true },
            ]
        },
        { path: "/statistics", component: StatisticsPage, props: true },
        { path: "/unauthorized", component: UnauthorizedPage, props: true },
        { path: "/session-expired", component: SessionExpiredPage, props: true },
        { path: "/:pathMatch(.*)", redirect: "/" }
    ]
});

export default router;
