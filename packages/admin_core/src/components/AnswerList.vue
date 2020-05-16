<template>
  <b-list-group v-if="item">
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
            @click="toggleRight(`${item.id}${i}`)"
          >
            <b-icon
              v-if="getState(`${item.id}${i}`, 'edit')"
              stacked
              icon="square"
            ></b-icon>
            <b-icon
              stacked
              :icon="answerRight(`${item.id}${i}`, i) ? 'check' : 'x'"
              :variant="answerRight(`${item.id}${i}`, i) ? 'success' : 'danger'"
            ></b-icon>
          </b-iconstack>
          <span v-if="!getState(`${item.id}${i}`, 'edit')">
            {{ `${i + 1}) ${answer.text}` }}
          </span>
          <b-form-input
            v-if="getState(`${item.id}${i}`, 'edit')"
            v-model="answerCopy.text"
            placeholder="Enter your answer"
          ></b-form-input>
        </div>
        <div class="d-flex align-items-center justify-content-around w-25">
          <span
            v-if="!getState(`${item.id}${i}`)"
            class="edit-btn px-1"
            @click="edit(`${item.id}${i}`, i)"
          >
            <b-icon icon="pencil"></b-icon>
            <span class="pl-1">Edit</span>
          </span>
          <span
            v-if="getState(`${item.id}${i}`)"
            class="ok-btn px-1"
            @click="ok(`${item.id}${i}`)"
          >
            <span class="pl-1">OK</span>
          </span>
          <span
            v-if="!getState(`${item.id}${i}`)"
            class="del-btn px-1"
            @click="remove(`${item.id}${i}`, i)"
          >
            <b-icon icon="trash"></b-icon>
            <span class="pl-1">Delete</span>
          </span>
          <span v-if="getState(`${item.id}${i}`)" class="del-btn px-1">
            <span class="pl-1" @click="cancel(`${item.id}${i}`)">
              Cancel
            </span>
          </span>
        </div>
      </div>
    </b-list-group-item>
    <b-list-group-item class="d-flex align-items-center justify-content-center">
      <b-button squared block variant="light" @click="add(item.id)">
        <b-icon icon="plus"></b-icon>
        Add answer
      </b-button>
    </b-list-group-item>
  </b-list-group>
</template>

<script>
import { mapActions, mapState } from 'vuex';
import { deepcopy } from '../helpers';

export default {
  props: {
    id: {
      type: String,
      default: '',
    },
  },

  data() {
    return {
      prevAdded: false,
      answerCopy: {},
      confirmMode: {},
    };
  },

  computed: {
    ...mapState({
      questions: state => state.tests.testsTomlData.questions,
    }),
    item() {
      return this.questions.find(v => v.id === this.id);
    },
  },

  methods: {
    ...mapActions({
      addNewAnswer: 'addNewAnswer',
      editAnswer: 'editAnswer',
      deleteAnswer: 'deleteAnswer',
    }),
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
    answerRight(id, i) {
      if (this.getState(id, 'edit')) {
        return this.answerCopy.right;
      }
      return this.item.answers[i].right;
    },
    toggleRight(id) {
      if (this.getState(id, 'edit')) {
        this.answerCopy.right = !this.answerCopy.right;
      }
    },
    ok(id) {
      this.prevAdded = false;
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
    add(id) {
      this.prevAdded = true;
      this.addNewAnswer({
        id,
        data: {
          text: '',
          right: false,
        },
      });
      this.edit(
        `${id}${this.item.answers.length - 1}`,
        this.item.answers.length - 1,
      );
    },
    edit(id, i) {
      this.answerCopy = deepcopy(this.item.answers[i]);
      this.toggleConfirmMode(id, 'edit');
      this.$confirm(`edit${id}`)
        .then(() => {
          this.toggleConfirmMode(id);
          this.editAnswer({
            id: this.id,
            i,
            data: this.answerCopy,
          });
        })
        .catch(() => {
          this.toggleConfirmMode(id);
          if (this.prevAdded) {
            this.deleteAnswer({ id: this.id, i });
            this.prevAdded = false;
          }
        });
    },
    remove(id, i) {
      this.toggleConfirmMode(id);
      this.$confirm(`remove${id}`)
        .then(() => {
          this.toggleConfirmMode(id);
          this.deleteAnswer({ id: this.id, i });
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
</style>
