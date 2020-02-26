<template>
  <v-app>
    <v-app-bar app dense>
      <v-toolbar-title>Stract Admin</v-toolbar-title>
    </v-app-bar>

    <v-content>
      <v-container fluid>
        <router-view></router-view>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
export default {
  name: 'App',
};
</script>
