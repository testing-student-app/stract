<template>
  <div class="wrapper">
    <div class="grid-users border-right">
      <h3 class="p-3 font-weight-bold">Connected Users</h3>
      <user-list />
    </div>
    <div class="grid-content">
      <test-list />
    </div>
  </div>
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
