<template>
  <b-row class="h-100">
    <b-col cols="4" class="h-100 overflow-auto border-right">
      <h3 class="font-weight-bold">Connected Users</h3>
      <user-list />
    </b-col>
    <b-col cols="8" class="h-100">
      <test-list />
    </b-col>
  </b-row>
</template>

<script>
import UserList from '../components/UserList.vue';
import TestList from '../components/TestList.vue';

export default {
  name: 'Main',

  components: {
    UserList,
    TestList,
  },
};
</script>
