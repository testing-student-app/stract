import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    serverLoaded: false,
    serverStatus: '',
    serverPort: 8081,
    users: [],
  },
  mutations: {
    TOGGLE_SERVER_LOADED(state) {
      state.serverLoaded = !state.serverLoaded;
    },
    SET_SERVER_STATUS(state, status) {
      state.serverStatus = status;
    },
    SET_SERVER_PORT(state, port) {
      state.serverPort = port;
    },
    SET_USERS(state, list) {
      state.users = list;
    },
  },
  actions: {
    toggleServerLoaded({ commit }) {
      commit('TOGGLE_SERVER_LOADED');
    },
    setServerStatus({ commit }, status) {
      commit('SET_SERVER_STATUS', status);
    },
    setServerPort({ commit }, port) {
      commit('SET_SERVER_PORT', port);
    },
    setUsers({ commit }, list) {
      commit('SET_USERS', list);
    },
  },
  modules: {},
});
