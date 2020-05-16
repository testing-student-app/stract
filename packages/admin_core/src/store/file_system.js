import tauri from 'tauri/api';

const state = {
  path: '',
};

const getters = {};

const mutations = {
  SET_PATH(state, path) {
    state.path = path;
  },
};

const actions = {
  newFile({ commit }) {
    commit('SET_PATH', '');
    commit('SET_TESTS', {
      questions: [],
    });
  },
  openFile({ commit }) {
    return tauri
      .promisified({
        cmd: 'openFile',
      })
      .then(internalTomlFile => {
        const { path, contents } = internalTomlFile;
        commit('SET_PATH', path);
        commit('SET_TESTS', contents);
        return internalTomlFile;
      });
  },
  saveFile({ state, commit, rootState }) {
    if (state.path) {
      return tauri
        .promisified({
          cmd: 'save',
          path: state.path,
          data: rootState.tests.testsTomlData,
        })
        .then(internalTomlFile => {
          const { path, contents } = internalTomlFile;
          commit('SET_PATH', path);
          commit('SET_TESTS', contents);
          return internalTomlFile;
        });
    }
    return tauri
      .promisified({
        cmd: 'saveAs',
        data: rootState.tests.testsTomlData,
      })
      .then(internalTomlFile => {
        const { path, contents } = internalTomlFile;
        commit('SET_PATH', path);
        commit('SET_TESTS', contents);
        return internalTomlFile;
      });
  },
  saveFileAs({ commit, rootState }) {
    return tauri
      .promisified({
        cmd: 'saveAs',
        data: rootState.tests.testsTomlData,
      })
      .then(internalTomlFile => {
        const { path, contents } = internalTomlFile;
        commit('SET_PATH', path);
        commit('SET_TESTS', contents);
        return internalTomlFile;
      });
  },
};

export default { state, getters, mutations, actions };
