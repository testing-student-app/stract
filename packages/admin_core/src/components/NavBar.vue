<template>
  <b-navbar variant="dark">
    <b-navbar-brand>
      <b-badge
        v-if="serverStatus === 'connected'"
        variant="success"
        class="text-wrap"
      >
        Connected, server running
      </b-badge>
      <b-badge
        v-else-if="serverStatus === 'started'"
        variant="warning"
        class="text-wrap"
      >
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
</template>

<script>
import { mapState } from 'vuex';

export default {
  name: 'NavBar',

  computed: {
    ...mapState({
      serverStatus: state => state.serverStatus,
    }),
  },
};
</script>

<style></style>
