<template>
  <b-overlay :show="serverLoaded" rounded="sm" class="h-100">
    <nav-bar />

    <b-container fluid class="main py-3">
      <router-view></router-view>
    </b-container>
  </b-overlay>
</template>

<script>
import { mapState, mapActions } from 'vuex';

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
    serverLoaded(val) {
      if (val) {
        this.setServerStatus('started');
        this.$ws.connect(`ws://localhost:${this.serverPort}`);
      }
    },
  },

  mounted() {
    window.tauri.listen('state', ({ state }) => {
      const { name, payload } = state;
      console.log(state);
      this.$store.dispatch(name, payload);
    });
  },

  methods: {
    ...mapActions(['setServerStatus']),
  },
};
</script>

<style lang="scss">
html,
body {
  height: 100%;
}

.main {
  height: calc(100% - 56px);
}
</style>
