const state = {
  testsTomlData: {},
};

const getters = {};

const mutations = {
  SET_TESTS(state, tomlData) {
    state.testsTomlData = tomlData;
  },
  SET_QUESTION(state, { id, data }) {
    const index = state.testsTomlData.questions.findIndex(v => v.id === id);
    state.testsTomlData.questions[index] = {
      ...data,
      answers: state.testsTomlData.questions[index].answers,
    };
  },
  PUSH_QUESTION(state, question) {
    state.testsTomlData.questions.push(question);
  },
  REMOVE_QUESTION(state, id) {
    state.testsTomlData.questions = state.testsTomlData.questions.filter(
      v => v.id !== id,
    );
  },
};

const actions = {
  setQuestion({ commit }, { id, data }) {
    commit('SET_QUESTION', { id, data });
  },
  pushQuestion({ commit }, question) {
    commit('PUSH_QUESTION', question);
  },
  removeQuestion({ commit }, id) {
    commit('REMOVE_QUESTION', id);
  },
  addNewAnswer({ state }, { id, data }) {
    const question = state.testsTomlData.questions.find(v => v.id === id);
    if (question && question.answers) {
      question.answers.push(data);
    }
  },
  editAnswer({ state }, { id, i, data }) {
    const question = state.testsTomlData.questions.find(v => v.id === id);
    if (question && question.answers) {
      question.answers[i] = data;
    }
  },
  deleteAnswer({ state }, { id, i }) {
    const question = state.testsTomlData.questions.find(v => v.id === id);
    if (question && question.answers) {
      question.answers.splice(i, 1);
    }
  },
};

export default { state, getters, mutations, actions };
