<template>
  <div class="d-flex flex-column h-100">
    <b-row class="h-100 overflow-auto">
      <b-col class="pt-2">
        <b-table class="tests-table" :fields="fields" :items="tests">
          <template v-slot:cell(question)="{ item, index }">
            <span>{{ `${index + 1} - ${item.question}` }}</span>
          </template>

          <template v-slot:cell(answers)="{ item }">
            <span>{{ item.answers.length }}</span>
          </template>

          <template v-slot:cell(actions)="row">
            <b-button size="sm" variant="info" @click="row.toggleDetails">
              {{ row.detailsShowing ? 'Hide' : 'Show' }} answers
            </b-button>
          </template>

          <template v-slot:row-details="{ item }">
            <AnswerList :item="item" />
          </template>
        </b-table>
      </b-col>
    </b-row>
  </div>
</template>

<script>
import { mapState } from 'vuex';

import AnswerList from './AnswerList.vue';

export default {
  name: 'TestList',

  components: {
    AnswerList,
  },

  data() {
    return {
      fields: [
        {
          key: 'question',
          label: 'Question',
        },
        {
          key: 'answers',
          label: 'Total answers',
        },
        'actions',
      ],
    };
  },

  computed: {
    ...mapState({
      tests: state => state.tests.testsTomlData.questions,
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
