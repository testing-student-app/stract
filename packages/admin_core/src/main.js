/* eslint-disable import/no-unresolved */
/* eslint-disable global-require */
import Vue from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';

import './plugins/bootstrap';
import './bootstrap_custom.scss';

import WebSocketPlugin from './plugins/websocket';

Vue.use(WebSocketPlugin, { store });

Vue.config.productionTip = false;

if (!window.tauri) {
  require('../../admin_tauri/src-tauri/tauri.js'); // change this path as needed
}

new Vue({
  router,
  store,
  render: h => h(App),
}).$mount('#app');
