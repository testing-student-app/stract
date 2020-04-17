<template>
  <div class="d-flex flex-column h-100">
    <b-row>
      <b-col class="border-bottom py-1">
        <b-button-toolbar
          aria-label="Toolbar with button groups and dropdown menu"
        >
          <b-dropdown variant="light">
            <template v-slot:button-content>
              File
            </template>
            <b-dropdown-item @click="newFile">New</b-dropdown-item>
            <b-dropdown-divider></b-dropdown-divider>
            <b-dropdown-item>Open File...</b-dropdown-item>
            <b-dropdown-divider></b-dropdown-divider>
            <b-dropdown-item>Save</b-dropdown-item>
            <b-dropdown-item>Save As...</b-dropdown-item>
          </b-dropdown>
        </b-button-toolbar>
      </b-col>
    </b-row>
    <b-row class="h-100 overflow-auto">
      <b-col class="pt-2">
        <b-table class="tests-table" :fields="fields" :items="items">
          <template v-slot:cell(answers)="row">
            <span>{{ row.item.answers.length }}</span>
          </template>

          <template v-slot:cell(actions)="row">
            <b-button size="sm" @click="row.toggleDetails">
              {{ row.detailsShowing ? 'Hide' : 'Show' }} Details
            </b-button>
          </template>

          <template v-slot:row-details="row">
            <pre>
              {{ `${JSON.stringify(row.item, null, 2)}` }}
            </pre>
          </template>
        </b-table>
      </b-col>
    </b-row>
  </div>
</template>

<script>
export default {
  name: 'TestList',

  data() {
    return {
      fields: [
        {
          key: 'question',
          label: 'Question',
        },
        {
          key: 'answers',
          label: 'Answers count',
        },
        {
          key: 'several_answers',
          label: 'Several Answers',
        },
        'actions',
      ],
      items: [],
    };
  },

  methods: {
    newFile() {
      window.tauri
        .promisified({
          cmd: 'newFile',
        })
        .then(({ payload }) => {
          this.items = payload;
        })
        .catch(({ payload }) => {
          this.$bvToast.toast(payload, {
            title: 'Error!',
            variant: 'danger',
            solid: true,
          });
        });
    },
  },
};
</script>

<style lang="scss" scoped>
::v-deep .tests-table {
  thead {
    th {
      position: sticky;
      top: 0px;
      background: #ffffff;
      z-index: 2;
    }
  }
}
</style>
