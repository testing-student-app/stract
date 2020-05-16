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

          <template
            v-slot:cell(actions)="{ toggleDetails, detailsShowing, item }"
          >
            <b-button
              size="sm"
              variant="info"
              class="mr-2"
              @click="toggleDetails"
            >
              {{ detailsShowing ? 'Hide' : 'Show' }} answers
            </b-button>
            <b-button
              size="sm"
              variant="danger"
              @click="deleteQuestion(item.id)"
              >Delete</b-button
            >
          </template>

          <template v-slot:row-details="{ item }">
            <AnswerList :id="item.id" />
          </template>
        </b-table>
      </b-col>
    </b-row>
  </div>
</template>

<script>
import { mapState, mapActions } from 'vuex';

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

  methods: {
    ...mapActions({
      removeQuestion: 'removeQuestion',
    }),
    deleteQuestion(id) {
      this.$bvModal
        .msgBoxConfirm(
          'Please confirm that you want to delete this question.',
          {
            title: 'Please Confirm',
            size: 'sm',
            buttonSize: 'sm',
            okVariant: 'danger',
            okTitle: 'YES',
            cancelTitle: 'NO',
            footerClass: 'p-2',
            hideHeaderClose: false,
            centered: true,
          },
        )
        .then(value => {
          if (value) {
            this.removeQuestion(id);
          }
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
