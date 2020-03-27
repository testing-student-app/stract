import Vue from 'vue';
import tauri from 'tauri/api';
import store from './store';

const WebsocketPlugin = new Vue({
  data() {
    return {
      socket: null,
    };
  },
  methods: {
    connect(port) {
      let socket = new WebSocket(`wss://localhost:${port}/ws/a`);

      socket.addEventListener('error', () => {
        store.dispatch('toggleServerLoaded');
        tauri
          .promisified({
            cmd: 'loadServer',
          })
          .then(({ status, port }) => {
            store.dispatch('setServerStatus', status);
            store.dispatch('toggleServerLoaded');
            socket = new WebSocket(`wss://localhost:${port}/ws/a`);
          });
      });

      socket.addEventListener('message', ({ data }) => {
        const { action, payload } = data;
        store.commit(action, payload);
      });
    },
  },
});

export default {
  install(Vue) {
    Vue.prototype.$websocket = WebsocketPlugin;
  },
};
