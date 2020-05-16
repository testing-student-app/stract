const WebSocketPlugin = ({ store }) => {
  /** @type {WebSocket} */
  let socket = null;
  const port = 8090;

  return {
    connect(url) {
      socket = new WebSocket(url.replace(':port', `:${port}`));

      socket.addEventListener('open', () => {
        store.dispatch('setServerStatus', 'connected');
      });

      socket.addEventListener('message', ({ data }) => {
        const { action, payload } = JSON.parse(data);
        store.dispatch(action, payload);
      });

      socket.addEventListener('error', () => {
        store.dispatch('setServerStatus', 'failed');
      });
    },

    emit(actionName, payload) {
      if (!socket) return;
      const data = {
        action: actionName,
        payload,
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
