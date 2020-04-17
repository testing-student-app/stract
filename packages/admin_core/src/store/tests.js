const state = {
  testsTomlData: [],
};

const getters = {};

const mutations = {
  SET_TESTS(state, tomlData) {
    state.testsTomlData = tomlData;
  },
};

const actions = {};

export default { state, getters, mutations, actions };
