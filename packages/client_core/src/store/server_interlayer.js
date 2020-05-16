const state = {
  serverStatus: '',
};

const mutations = {
  SET_SERVER_STATUS(state, status) {
    state.serverStatus = status;
  },
};

const actions = {
  setServerStatus({ commit }, status) {
    commit('SET_SERVER_STATUS', status);
  },
};

export default {
  state,
  actions,
  mutations,
};
