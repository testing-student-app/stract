<template>
  <b-overlay :show="serverLoaded" rounded="sm" class="h-100">
    <nav-bar />

    <b-container fluid class="main py-3">
      <router-view></router-view>
    </b-container>
  </b-overlay>
</template>

<script>
import { mapActions, mapState } from 'vuex';

import NavBar from './components/NavBar.vue';

export default {
  name: 'App',

  components: {
    NavBar,
  },

  computed: {
    ...mapState({
      serverLoaded: state => state.serverLoaded,
    }),
  },

  created() {
    // window.tauri
    //   .promisified({
    //     cmd: 'loadServer',
    //   })
    //   .then(({ status }) => {
    //     this.setServerStatus(status);
    //     this.toggleServerLoaded();
    //   });
    this.$ws.connect('ws://127.0.0.1:8081/ws/a');
  },

  methods: {
    ...mapActions(['toggleServerLoaded', 'setServerStatus']),
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
