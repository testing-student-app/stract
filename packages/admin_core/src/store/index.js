import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    serverLoaded: 'false',
    serverStatus: '',
    serverPort: '8081',
    users: [],
  },
  mutations: {
    SET_SERVER_LOADED(state) {
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
    server_loaded({ commit }, payload) {
      console.log('actions', 'server_loaded', payload);
      commit('SET_SERVER_LOADED', payload);
    },
    setServerStatus({ commit }, status) {
      commit('SET_SERVER_STATUS', status);
    },
    server_port({ commit }, port) {
      console.log('actions', 'server_port', port);
      commit('SET_SERVER_PORT', port);
    },
    setUsers({ commit }, list) {
      commit('SET_USERS', list);
    },
  },
  modules: {},
});
