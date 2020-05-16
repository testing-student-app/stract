import Vue from 'vue';
import Vuex from 'vuex';
import fileSystem from './file_system.js';
import tests from './tests.js';
import serverInterlayer from './server_interlayer.js';

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    fileSystem,
    tests,
    serverInterlayer,
  },
});
