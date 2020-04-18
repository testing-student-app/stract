<template>
  <div class="wrapper">
    <nav-bar />
    <router-view></router-view>
  </div>
</template>

<script>
import { mapState, mapActions } from 'vuex';
import tauri from 'tauri/api';

import NavBar from './components/NavBar.vue';

export default {
  name: 'App',

  components: {
    NavBar,
  },

  computed: {
    ...mapState({
      serverLoaded: state => state.serverLoaded,
      serverPort: state => state.serverPort,
    }),
  },

  watch: {
    serverLoaded: {
      handler(val) {
        if (val) {
          this.setServerStatus('started');
          this.$ws.connect(`ws://127.0.0.1:${this.serverPort}/ws/a`);
        }
      },
      immediate: true,
    },
  },

  beforeCreate() {
    tauri.listen('state', ({ payload: state }) => {
      const { name, payload } = state;
      this.$store.dispatch(name, payload);
    });
  },

  methods: {
    ...mapActions(['setServerStatus']),
  },
};
</script>

<style lang="scss">
* {
  box-sizing: border-box;
  padding: 0;
  margin: 0;
}

.grid-header {
  grid-area: header;
}

.grid-content {
  grid-area: content;
  overflow-y: auto;
}

.grid-users {
  grid-area: users;
}

.wrapper {
  width: 100vw;
  height: 100vh;
  display: grid;
  grid-template-columns: 1fr 3fr;
  grid-template-rows: max-content;
  grid-template-areas:
    'header header'
    'users  content';
  overflow: hidden;
}
</style>
