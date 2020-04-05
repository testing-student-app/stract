/* eslint-disable import/no-unresolved */
/* eslint-disable global-require */
import Vue from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';

import './plugins/bootstrap';
import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap-vue/dist/bootstrap-vue.css';

import WebSocketPlugin from './plugins/websocket';

Vue.use(WebSocketPlugin, { store });

Vue.config.productionTip = false;

new Vue({
  router,
  store,
  render: h => h(App),
}).$mount('#app');
