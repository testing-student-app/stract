import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    serverLoaded: false,
    serverStatus: '',
  },
  mutations: {
    TOGGLE_SERVER_LOADED(state) {
      state.serverLoaded = !state.serverLoaded;
    },
    SET_SERVER_STATUS(state, status) {
      state.serverStatus = status;
    },
  },
  actions: {
    toggleServerLoaded({ commit }) {
      commit('TOGGLE_SERVER_LOADED');
    },
    setServerStatus({ commit }, status) {
      commit('SET_SERVER_STATUS', status);
    },
  },
  modules: {},
});
