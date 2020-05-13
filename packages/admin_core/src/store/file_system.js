import tauri from 'tauri/api';

const state = {};

const getters = {};

const mutations = {};

const actions = {
  openFile({ commit }) {
    window.log('openfile');
    return tauri
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
