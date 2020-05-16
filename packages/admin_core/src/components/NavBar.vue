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

    <b-navbar-nav class="ml-auto">
      <b-button-toolbar
        aria-label="Toolbar with button groups and dropdown menu"
      >
        <new-question-modal class="mr-2" />
        <b-dropdown variant="primary" right>
          <template v-slot:button-content>
            File
          </template>
          <b-dropdown-item @click="newFile">New</b-dropdown-item>
          <b-dropdown-divider></b-dropdown-divider>
          <b-dropdown-item @click="openFileLocal">Open File...</b-dropdown-item>
          <b-dropdown-divider></b-dropdown-divider>
          <b-dropdown-item @click="saveFileLocal">Save</b-dropdown-item>
          <b-dropdown-item @click="saveFileAsLocal">Save As...</b-dropdown-item>
        </b-dropdown>
      </b-button-toolbar>
    </b-navbar-nav>
  </b-navbar>
</template>

<script>
import { mapState, mapActions } from 'vuex';
import NewQuestionModal from './NewQuestionModal.vue';

export default {
  name: 'NavBar',

  components: {
    NewQuestionModal,
  },

  computed: {
    ...mapState({
      serverStatus: state => state.serverInterlayer.serverStatus,
    }),
  },

  methods: {
    ...mapActions(['newFile', 'openFile', 'saveFile', 'saveFileAs']),
    openFileLocal() {
      this.openFile().catch(errorMessage => {
        let message;
        if (!errorMessage) {
          message = errorMessage;
        } else {
          message = 'Something went terribly wrong with opening file';
        }
        this.$bvToast.toast(message, {
          title: 'File System Error!',
          variant: 'danger',
          solid: true,
        });
      });
    },
    saveFileLocal() {
      this.saveFile().catch(errorMessage => {
        let message;
        if (!errorMessage) {
          message = errorMessage;
        } else {
          message = 'Something went terribly wrong with saving file';
        }
        this.$bvToast.toast(message, {
          title: 'File System Error!',
          variant: 'danger',
          solid: true,
        });
      });
    },
    saveFileAsLocal() {
      this.saveFileAs().catch(errorMessage => {
        let message;
        if (!errorMessage) {
          message = errorMessage;
        } else {
          message = 'Something went terribly wrong with saving file as';
        }
        this.$bvToast.toast(message, {
          title: 'File System Error!',
          variant: 'danger',
          solid: true,
        });
      });
    },
  },
};
</script>

<style></style>
