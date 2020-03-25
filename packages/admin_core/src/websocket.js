import Vue from 'vue';
import tauri from 'tauri/api';

const WebsocketPlugin = new Vue({
  data() {
    return {
      port: 8081,
      socket: null,
    };
  },
  methods: {
    connect() {
      const socket = new WebSocket(`wss://localhost:${this.port}/ws/a`);
    },
  },
});

export default {
  install(Vue) {
    Vue.prototype.$websocket = WebsocketPlugin;
  },
};
