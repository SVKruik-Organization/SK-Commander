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
import PreferencesPage from "./components/PreferencesPage.vue";
import AccountSettings from "./components/Preferences/AccountSettings.vue";
import ApplicationSettings from "./components/Preferences/ApplicationSettings.vue";
import LinkSettings from "./components/Preferences/LinkSettings.vue";
import PlansPage from "./components/Preferences/PlansPage.vue";
import OperatorPage from "./components/OperatorPage.vue";
import SupportPage from "./components/SupportPage.vue";

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
                { path: "information", component: InformationSettings, props: true }
            ]
        },
        {
            path: "/preferences", component: PreferencesPage, props: true, children: [
                { path: "", redirect: "/preferences/account" },
                { path: "account", component: AccountSettings, props: true },
                { path: "application", component: ApplicationSettings, props: true },
                { path: "links", component: LinkSettings, props: true }
            ]
        },
        { path: "/statistics", component: StatisticsPage, props: true },
        { path: "/plans", component: PlansPage, props: true },
        { path: "/operators", component: OperatorPage, props: true },
        { path: "/support", component: SupportPage, props: true },
        { path: "/unauthorized", component: UnauthorizedPage, props: true },
        { path: "/session-expired", component: SessionExpiredPage, props: true },
        { path: "/:pathMatch(.*)", redirect: "/" }
    ]
});

export default router;
