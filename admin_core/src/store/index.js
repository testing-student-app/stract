import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    serverLoading: false,
  },
  mutations: {
    TOGGLE_SERVER_LOADING(state) {
      state.serverLoading = !state.serverLoading;
    },
  },
  actions: {
    toggleServerLoading({ commit }) {
      commit('TOGGLE_SERVER_LOADING');
    },
  },
  modules: {},
});
