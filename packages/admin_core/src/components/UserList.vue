<template>
  <b-list-group>
    <b-list-group-item v-for="u in users" :key="u.remote_addr">
      {{ u.name || u.remote_addr }}
    </b-list-group-item>
  </b-list-group>
</template>

<script>
import { mapState } from 'vuex';

export default {
  name: 'UserList',

  computed: {
    ...mapState({
      users: state => state.serverInterlayer.users,
    }),
  },
};
</script>
