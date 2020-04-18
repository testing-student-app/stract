<template>
  <b-list-group class="p-3">
    <b-list-group-item v-for="u in users" :key="u">{{ u }}</b-list-group-item>
  </b-list-group>
</template>

<script>
import { mapState } from 'vuex';

export default {
  name: 'UserList',

  computed: {
    ...mapState({
      users: state => state.users,
    }),
  },
};
</script>
