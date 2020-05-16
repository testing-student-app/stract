import Vue from 'vue';
import Vuex from 'vuex';
import serverInterlayer from './server_interlayer.js';

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    serverInterlayer,
  },
});
