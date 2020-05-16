import tauri from 'tauri/api';

const state = {
  serverLoaded: 'false',
  serverStatus: '',
  users: [],
};

const mutations = {
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
};

const actions = {
  startServer({ commit }, port) {
    return tauri
      .promisified({
        cmd: 'startServer',
        port: port.toString(),
      })
      .then(data => {
        commit('SET_SERVER_LOADED', true);
        commit('SET_SERVER_STATUS', 'started');
        return data;
      })
      .catch(() => {
        commit('SET_SERVER_LOADED', false);
        commit('SET_SERVER_STATUS', 'failed');
      });
  },
  checkServer({ commit }, port) {
    return tauri
      .promisified({
        cmd: 'checkServer',
        port: port.toString(),
      })
      .then(data => {
        commit('SET_SERVER_STATUS', 'started');
        return data;
      })
      .catch(() => {
        commit('SET_SERVER_STATUS', 'failed');
      });
  },
  setServerStatus({ commit }, status) {
    commit('SET_SERVER_STATUS', status);
  },
  setUsers({ commit }, list) {
    commit('SET_USERS', list);
  },
};

export default {
  state,
  actions,
  mutations,
};
