// store.js
import { createStore } from 'vuex';

export default createStore({
    state: {
        activeGuild: null,
    },
    mutations: {
        setActiveGuild(state, guild) {
            state.activeGuild = guild;
        },
    },
    actions: {
        setActiveGuild({ commit }, guild) {
            commit('setActiveGuild', guild);
        },
    },
    getters: {
        getActiveGuild: (state) => state.activeGuild,
    },
});
