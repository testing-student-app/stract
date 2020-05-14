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

          <template v-slot:row-details="{ item, index }">
            <b-list-group>
              <b-list-group-item
                v-for="(answer, i) in item.answers"
                :key="i"
                class="answer-item"
              >
                <div class="d-flex justify-content-between">
                  <div class="d-flex align-items-center w-75">
                    <b-iconstack
                      font-scale="2"
                      class="mr-2"
                      @click="toggleRight(`${index}${i}`, answer)"
                    >
                      <b-icon
                        v-if="getState(`${index}${i}`, 'edit')"
                        stacked
                        icon="square"
                      ></b-icon>
                      <b-icon
                        stacked
                        :icon="answer.right ? 'check' : 'x'"
                        :variant="answer.right ? 'success' : 'danger'"
                      ></b-icon>
                    </b-iconstack>
                    <span v-if="!getState(`${index}${i}`, 'edit')">
                      {{ `${i + 1}) ${answer.text}` }}
                    </span>
                    <b-form-input
                      v-if="getState(`${index}${i}`, 'edit')"
                      v-model="answer.text"
                      placeholder="Enter your answer"
                    ></b-form-input>
                  </div>
                  <div
                    class="d-flex align-items-center justify-content-around w-25"
                  >
                    <span
                      v-if="!getState(`${index}${i}`)"
                      class="edit-btn px-1"
                      @click="edit(`${index}${i}`, i)"
                    >
                      <b-icon icon="pencil"></b-icon>
                      <span class="pl-1">Edit</span>
                    </span>
                    <span
                      v-if="getState(`${index}${i}`)"
                      class="ok-btn px-1"
                      @click="ok(`${index}${i}`)"
                    >
                      <span class="pl-1">OK</span>
                    </span>
                    <span
                      v-if="!getState(`${index}${i}`)"
                      class="del-btn px-1"
                      @click="remove(`${index}${i}`, i, item.answers)"
                    >
                      <b-icon icon="trash"></b-icon>
                      <span class="pl-1">Delete</span>
                    </span>
                    <span v-if="getState(`${index}${i}`)" class="del-btn px-1">
                      <span class="pl-1" @click="cancel(`${index}${i}`)">
                        Cancel
                      </span>
                    </span>
                  </div>
                </div>
              </b-list-group-item>
              <b-list-group-item
                class="d-flex align-items-center justify-content-center"
              >
                <b-button
                  squared
                  block
                  variant="light"
                  @click="add(index, item.answers)"
                >
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
      confirmMode: {},
    };
  },

  computed: {
    ...mapState({
      tests: state => state.tests.testsTomlData,
    }),
  },

  methods: {
    getState(id, state = 'confirm') {
      if (this.confirmMode[id]) {
        return this.confirmMode[id][state];
      }
      return false;
    },
    toggleConfirmMode(id, mode = 'confirm') {
      let newState = {
        [id]: {
          [mode]: this.confirmMode[id]
            ? !this.confirmMode[id][mode]
            : !this.confirmMode[id],
          confirm: this.confirmMode[id]
            ? !this.confirmMode[id].confirm
            : !this.confirmMode[id],
        },
      };
      if (mode === 'confirm') {
        newState = {
          [id]: {
            [mode]: this.confirmMode[id]
              ? !this.confirmMode[id][mode]
              : !this.confirmMode[id],
          },
        };
      }
      this.confirmMode = {
        ...this.confirmMode,
        ...newState,
      };
    },
    toggleRight(id, answer) {
      if (this.getState(id, 'edit')) {
        answer.right = !answer.right;
      }
    },
    ok(id) {
      if (this.getState(id, 'edit')) {
        this.$confirm('ok', `edit${id}`);
      } else if (this.getState(id)) {
        this.$confirm('ok', `remove${id}`);
      }
    },
    cancel(id) {
      if (this.getState(id, 'edit')) {
        this.$confirm('cancel', `edit${id}`);
      } else if (this.getState(id)) {
        this.$confirm('cancel', `remove${id}`);
      }
    },
    add(id, answers) {
      answers.push({
        text: '',
        right: false,
      });
      this.edit(`${id}${answers.length - 1}`, answers.length - 1);
    },
    edit(id, i) {
      this.toggleConfirmMode(id, 'edit');
      this.$confirm(`edit${id}`)
        .then(() => {
          this.toggleConfirmMode(id);
          console.log(i);
        })
        .catch(() => {
          this.toggleConfirmMode(id);
        });
    },
    remove(id, i, answers) {
      this.toggleConfirmMode(id);
      this.$confirm(`remove${id}`)
        .then(() => {
          this.toggleConfirmMode(id);
          answers.splice(i, 1);
        })
        .catch(() => {
          this.toggleConfirmMode(id);
        });
    },
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
    .ok-btn {
      &:hover {
        @extend %btn;
        color: #28a745;
        text-decoration: underline;
      }
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
