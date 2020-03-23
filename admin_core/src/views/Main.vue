<template>
  <div>
    <v-container v-if="!serverLoaded" fluid>
      <loader></loader>
    </v-container>
    <v-container v-else fluid>
      <v-row align-content="center" align="stretch">
        <v-col cols="12">
          <v-text-field label="Message" single-line filled></v-text-field>
        </v-col>
      </v-row>
      <v-row align-content="space-between" align="stretch">
        <v-col cols="6">
          <v-card class="mx-auto">
            <v-card-title>Connected Users</v-card-title>

            <v-card-text>1,000 miles of wonder</v-card-text>
          </v-card>
        </v-col>
        <v-col cols="6">
          <v-card class="mx-auto">
            <v-textarea
              outlined
              name="input-7-4"
              label="Server Output"
            ></v-textarea>

            <v-card-actions>
              <v-btn text>Clear</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<script>
import { mapState } from 'vuex';
import Loader from '../components/Loader.vue';

export default {
  name: 'Main',
  components: { Loader },
  computed: mapState({
    serverLoaded: state => state.serverLoaded,
  }),
};
</script>
