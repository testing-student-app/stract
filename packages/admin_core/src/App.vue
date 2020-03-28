<template>
  <b-overlay :show="serverLoaded" rounded="sm">
    <b-navbar variant="dark" class="mb-3">
      <b-navbar-brand>
        <b-badge v-if="serverStatus === 'started'" class="text-wrap">
          Not connected, server running
        </b-badge>
        <b-badge v-else variant="danger" class="text-wrap">
          Not connected, server not running
        </b-badge>
      </b-navbar-brand>

      <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

      <b-collapse id="nav-collapse" is-nav>
        <!-- Right aligned nav items -->
        <b-navbar-nav class="ml-auto"> </b-navbar-nav>
      </b-collapse>
    </b-navbar>
    <b-container fluid>
      <router-view></router-view>
    </b-container>
  </b-overlay>
</template>

<script>
import tauri from 'tauri/api';
import { mapActions, mapState } from 'vuex';

export default {
  name: 'App',
  computed: {
    ...mapState({
      serverLoaded: state => state.serverLoaded,
      serverStatus: state => state.serverStatus,
    }),
  },
  created() {
    tauri
      .promisified({
        cmd: 'loadServer',
      })
      .then(({ status, port }) => {
        this.setServerPort(port);
        this.setServerStatus(status);
        this.toggleServerLoaded();
      });
    this.$ws.connect('ws://127.0.0.1:8081/ws/a');
  },
  methods: {
    ...mapActions(['toggleServerLoaded', 'setServerStatus', 'setServerPort']),
  },
};
</script>

<style lang="scss">
html {
  overflow-y: hidden !important;
}
</style>
