<template>
  <h1>HI</h1>
  <!-- <v-app>
    <v-app-bar app dense>
      <v-toolbar-title>Stract Admin</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-toolbar-title v-if="serverStatus === 'started'"
        >Not connected, server running</v-toolbar-title
      >
    </v-app-bar>

    <v-content>
      <v-container fluid>
        <router-view></router-view>
      </v-container>
    </v-content>
  </v-app> -->
</template>

<script>
import tauri from 'tauri/api';
import { mapActions, mapState } from 'vuex';

export default {
  name: 'App',
  computed: {
    ...mapState({
      serverStatus: state => state.serverStatus,
    }),
  },
  created() {
    tauri
      .promisified({
        cmd: 'loadServer',
      })
      .then(({ status }) => {
        this.setServerStatus(status);
        this.toggleServerLoaded();
      });
  },
  methods: {
    ...mapActions(['toggleServerLoaded', 'setServerStatus']),
  },
};
</script>

<style lang="scss">
html {
  overflow-y: hidden !important;
}
</style>
