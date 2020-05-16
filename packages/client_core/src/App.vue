<template>
  <div>
    <nav-bar />

    <b-container fluid class="main py-3">
      <router-view></router-view>
    </b-container>
  </div>
</template>

<script>
import tauri from 'tauri/api';
import NavBar from './components/NavBar.vue';

export default {
  name: 'App',

  components: {
    NavBar,
  },

  computed: {},

  watch: {},

  mounted() {
    this.$ws.connect(`ws://127.0.0.1:port/ws/c`);
  },

  beforeCreate() {
    tauri.listen('state', ({ payload: state }) => {
      const { name, payload } = state;
      this.$store.dispatch(name, payload);
    });
  },
};
</script>

<style lang="scss"></style>
