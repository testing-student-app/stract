const ConfirmPlugin = () => {
  const state = new Map();
  return (...args) => {
    if (args[0] === 'ok' && state.has(args[1])) {
      state.get(args[1]).ok();
      state.delete(args[1]);
      return undefined;
    }
    if (args[0] === 'cancel' && state.has(args[1])) {
      state.get(args[1]).cancel();
      state.delete(args[1]);
      return undefined;
    }

    return new Promise((resolve, reject) => {
      state.set(args[0], {
        ok: resolve,
        cancel: reject,
      });
    });
  };
};

export default {
  install(Vue) {
    const $vm = Vue;
    $vm.prototype.$confirm = ConfirmPlugin();
  },
};
