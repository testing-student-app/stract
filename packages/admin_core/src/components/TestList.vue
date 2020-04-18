<template>
  <div class="d-flex flex-column h-100">
    <b-row class="h-100">
      <b-col class="pt-2">
        <b-table class="tests-table" :fields="fields" :items="tests">
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
import { mapState } from 'vuex';

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
    };
  },
  computed: {
    ...mapState({
      tests: state => state.tests.testsTomlData,
    }),
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
