<template>
  <b-row class="h-100">
    <b-col
      cols="12"
      class="h-100 d-flex align-items-center justify-content-center"
    >
      <b-input-group class="mb-3" prepend="Name">
        <b-form-input v-model="name"></b-form-input>
        <b-input-group-append>
          <b-button text="Ok" variant="primary" @click="ok">Ok</b-button>
        </b-input-group-append>
      </b-input-group>
    </b-col>
  </b-row>
</template>

<script>
export default {
  name: 'Main',
  components: {},
  data() {
    return {
      name: '',
    };
  },
  methods: {
    ok() {
      this.$ws.emit('setname', { name: this.name });
    },
  },
};
</script>
