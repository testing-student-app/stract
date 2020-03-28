<template>
  <div>
    <b-container fluid>
      <b-row>
        <b-col cols="4">
          <h3 class="font-weight-bold">Connected Users</h3>
          <b-list-group>
            <b-list-group-item v-for="u in users" :key="u">{{
              u
            }}</b-list-group-item>
          </b-list-group>
        </b-col>
        <b-col cols="8"> </b-col>
      </b-row>
    </b-container>
  </div>
</template>

<script>
import { mapState } from 'vuex';

export default {
  name: 'Main',

  computed: {
    ...mapState({
      users: state => state.users,
    }),
  },
};
</script>
