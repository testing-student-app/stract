// import tauri from 'tauri/api';

const WebSocketPlugin = ({ store }) => {
  /** @type {WebSocket} */
  let socket = null;

  return {
    connect(url) {
      socket = new WebSocket(url);

      socket.addEventListener('message', ({ data }) => {
        const { action, payload } = JSON.parse(data);
        store.dispatch(action, payload);
      });
    },

    emit(actionName, payload) {
      if (!socket) return;
      const data = {
        action: actionName,
        data: payload,
      };
      socket.send(JSON.stringify(data));
    },
  };
};

export default {
  install(Vue, options) {
    const $vm = Vue;
    $vm.prototype.$ws = WebSocketPlugin(options);
  },
};
