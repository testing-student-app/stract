<template>
  <div>
    <b-button variant="info" @click="newFileNotCreated">New question</b-button>

    <b-modal
      id="NewQuestion"
      ref="new-question-modal"
      title="Create new question"
      size="lg"
      @shown="shown"
      @hidden="closeWithDestroy"
    >
      <b-form-group
        id="NewQuestionGroup"
        label="Enter your new question"
        label-for="QuestionName"
        invalid-feedback="Question is required"
        :state="questionState"
      >
        <b-form-input
          id="QuestionName"
          v-model="newQuestion.question"
          :state="questionState"
          trim
        ></b-form-input>
      </b-form-group>
      <template v-slot:modal-footer>
        <b-button
          variant="secondary"
          class="mr-2 float-right"
          @click="closeWithDestroy"
        >
          Close
        </b-button>
        <b-button
          :disabled="!questionState"
          variant="primary"
          class="float-right"
          @click="confirm"
        >
          Confirm
        </b-button>
      </template>
      <answers-list :id="newQuestion.id" />
    </b-modal>
  </div>
</template>

<script>
import { nanoid } from 'nanoid';
import { mapActions, mapState } from 'vuex';
import AnswersList from './AnswerList.vue';
import { deepcopy } from '../helpers.js';

export default {
  components: {
    AnswersList,
  },
  data() {
    return {
      newQuestion: {
        id: nanoid(),
        question: '',
        answers: [],
      },
    };
  },
  computed: {
    ...mapState({
      testsTomlData: state => state.tests.testsTomlData,
    }),
    questionState() {
      return this.newQuestion.question.length > 0;
    },
  },
  methods: {
    ...mapActions({
      newFile: 'newFile',
      setQuestion: 'setQuestion',
      pushQuestion: 'pushQuestion',
      removeQuestion: 'removeQuestion',
    }),
    shown() {
      const copy = deepcopy(this.newQuestion);
      this.pushQuestion(copy);
    },
    newFileNotCreated() {
      if (
        !Object.prototype.hasOwnProperty.call(this.testsTomlData, 'questions')
      ) {
        this.newFile();
      }
      this.$refs['new-question-modal'].show();
    },
    clear() {
      this.newQuestion = {
        id: nanoid(),
        question: '',
        answers: [],
      };
    },
    closeWithDestroy() {
      this.removeQuestion(this.newQuestion.id);
      this.close();
    },
    close() {
      this.$refs['new-question-modal'].hide();
    },
    confirm() {
      if (!this.questionState) {
        return;
      }
      this.setQuestion({ id: this.newQuestion.id, data: this.newQuestion });
      this.close();
      this.clear();
    },
  },
};
</script>

<style lang="scss" scoped></style>
