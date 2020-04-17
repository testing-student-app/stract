const state = {};

const getters = {};

const mutations = {};

const actions = {
  openFile({ commit }) {
    return window.tauri
      .promisified({
        cmd: 'openFile',
      })
      .then(tomlData => {
        commit('SET_TESTS', tomlData?.data);
        return tomlData;
      });
  },
};

export default { state, getters, mutations, actions };
