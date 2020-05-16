<template>
  <b-navbar variant="dark">
    <b-navbar-brand>
      <b-badge
        v-if="serverStatus === 'connected'"
        variant="success"
        class="text-wrap"
      >
        Connected
      </b-badge>
      <b-badge v-else variant="danger" class="text-wrap">
        Not connected
      </b-badge>
    </b-navbar-brand>
  </b-navbar>
</template>

<script>
import { mapState } from 'vuex';

export default {
  name: 'NavBar',

  computed: {
    ...mapState({
      serverStatus: state => state.serverInterlayer.serverStatus,
    }),
  },
};
</script>

<style></style>
