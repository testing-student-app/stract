const state = {
  testsTomlData: [
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [
        {
          text: '1) 123',
          right: false,
        },
        {
          text: '1) 1234',
          right: false,
        },
        {
          text: '1) 1235',
          right: true,
        },
        {
          text: '1) 1236',
          right: false,
        },
      ],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: true,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
    {
      question: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit.',
      several_answers: false,
      answers: [],
    },
  ],
};

const getters = {};

const mutations = {
  SET_TESTS(state, tomlData) {
    state.testsTomlData = tomlData;
  },
};

const actions = {};

export default { state, getters, mutations, actions };
