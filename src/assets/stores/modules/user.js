export default {
    state: {
        activeUser: null,
    },
    mutations: {
        setActiveUser(state, user) {
            state.activeUser = user;
        },
    },
    actions: {
        setActiveUser({ commit }, user) {
            commit('setActiveUser', user);
        },
    },
    getters: {
        getActiveUser: (state) => state.activeUser,
    },
}