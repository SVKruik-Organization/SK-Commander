import { createStore } from 'vuex';
import activeGuild from './modules/activeGuild';
import user from './modules/user';

export default createStore({
    modules: {
        activeGuild: activeGuild,
        user: user
    }
});
