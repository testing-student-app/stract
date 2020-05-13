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
            <b-button size="sm" @click="row.toggleDetails">
              {{ row.detailsShowing ? 'Hide' : 'Show' }} Details
            </b-button>
          </template>

          <template v-slot:row-details="{ item }">
            <b-list-group>
              <b-list-group-item
                v-for="(answer, i) in item.answers"
                :key="i"
                class="answer-item"
              >
                <div class="d-flex justify-content-between">
                  <div class="d-flex align-items-center">
                    <b-icon
                      font-scale="2"
                      :icon="answer.right ? 'check' : 'x'"
                      :variant="answer.right ? 'success' : 'danger'"
                    ></b-icon>
                    <span class="pl-2">
                      {{ `${i + 1}) ${answer.text}` }}
                    </span>
                  </div>
                  <div class="d-flex align-items-center">
                    <span class="edit-btn px-1">
                      <b-icon icon="pencil"></b-icon>
                      <span class="pl-1">Edit</span>
                    </span>
                    <span class="del-btn px-1">
                      <b-icon icon="trash"></b-icon>
                      <span class="pl-1">Delete</span>
                    </span>
                  </div>
                </div>
              </b-list-group-item>
              <b-list-group-item
                class="d-flex align-items-center justify-content-center"
              >
                <b-button squared block variant="light">
                  <b-icon icon="plus"></b-icon>
                  Add answer
                </b-button>
              </b-list-group-item>
            </b-list-group>
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
          label: 'Total answers',
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
%btn {
  cursor: pointer;
  text-decoration: underline;
}

::v-deep .tests-table {
  thead {
    th {
      position: sticky;
      top: 0px;
      background: #ffffff;
      z-index: 2;
    }
  }
  .answer-item {
    &:hover {
      transform: scale(1.005);
      margin: 0.5px;
      box-shadow: 0px 0px 5px 0px rgba(0, 0, 0, 0.5);
    }
    .edit-btn {
      &:hover {
        @extend %btn;
        color: #ffc107;
        text-decoration: underline;
      }
    }
    .del-btn {
      &:hover {
        @extend %btn;
        color: #dc3545;
      }
    }
  }
}
</style>
