<template>
  <b-navbar variant="dark">
    <b-navbar-brand>
      <b-badge
        v-if="serverStatusLocal === 'connected'"
        variant="success"
        class="text-wrap"
      >
        Connected, server running
      </b-badge>
      <b-badge
        v-else-if="serverStatusLocal === 'started'"
        variant="warning"
        class="text-wrap"
      >
        Not connected, server running
      </b-badge>
      <b-badge v-else variant="danger" class="text-wrap">
        Not connected, server not running
      </b-badge>
    </b-navbar-brand>

    <b-navbar-nav class="ml-auto">
      <b-button-toolbar
        aria-label="Toolbar with button groups and dropdown menu"
      >
        <b-dropdown variant="primary" right>
          <template v-slot:button-content>
            File
          </template>
          <b-dropdown-item>New</b-dropdown-item>
          <b-dropdown-divider></b-dropdown-divider>
          <b-dropdown-item @click="openFileLocal">Open File...</b-dropdown-item>
          <b-dropdown-divider></b-dropdown-divider>
          <b-dropdown-item>Save</b-dropdown-item>
          <b-dropdown-item>Save As...</b-dropdown-item>
        </b-dropdown>
      </b-button-toolbar>
    </b-navbar-nav>
  </b-navbar>
</template>

<script>
import { mapState, mapActions } from 'vuex';

export default {
  name: 'NavBar',

  computed: {
    ...mapState({
      serverStatus: state => state.serverInterlayer.serverStatus,
    }),
    serverStatusLocal() {
      console.log(this.serverStatus);
      return this.serverStatus;
    },
  },

  methods: {
    ...mapActions(['openFile']),
    openFileLocal() {
      this.openFile().catch(errorMessage => {
        let message;
        if (!errorMessage) {
          message = errorMessage;
        } else {
          message = 'Something went terribly wrong';
        }
        this.$bvToast.toast(message, {
          title: 'File System Error!',
          variant: 'danger',
          solid: true,
        });
      });
    },
  },
};
</script>

<style></style>
