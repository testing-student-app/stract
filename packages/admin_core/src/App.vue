<template>
  <div>
    <nav-bar />

    <b-container fluid class="main py-3">
      <router-view></router-view>
    </b-container>
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

  watch: {},

  mounted() {
    this.$ws.connect(`ws://127.0.0.1:port/ws/a`);
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
html,
body {
  height: 100%;
}

.main {
  height: calc(100% - 56px);
  position: absolute;
}
</style>
