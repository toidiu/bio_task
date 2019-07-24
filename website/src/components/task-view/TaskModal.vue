<template>
  <div class="modal" v-if="modalTask" :class="{ 'is-active': modalActive }">
    <div class="modal-background" v-on:click="closeModal"></div>
    <div class="modal-card">
      <div class="modal-card-head">
        <p class="modal-card-title">{{ modalTask.title }}</p>
        <button
          class="delete"
          aria-label="close"
          v-on:click="closeModal"
        ></button>
      </div>
      <section class="modal-card-body">
        <p>{{ modalTask.statusKey }}</p>
        <p>{{ modalTask.deadlineDate }}</p>
        <p>{{ modalTask.description }}</p>
      </section>
      <footer class="modal-card-foot">
        <button class="button is-primary is-outlined">submit</button>
        <button class="button is-danger is-outlined">cancel</button>
      </footer>
    </div>
  </div>
</template>

<script lang="ts">
import { TaskResp } from "./models";
import Vue from "vue";
import ScrollView from "./ScrollView.vue";

export default Vue.extend({
  components: {
    ScrollView
  },
  props: {
    modalTask: Object
  },
  data: function() {
    return {};
  },
  methods: {
    closeModal: function() {
      this.$emit("close-modal-event");
    },
    editTask: function(t) {
      console.log(t);
    }
  },
  computed: {
    modalActive: function() {
      return this.modalTask != null;
    }
  }
});
</script>

<style lang="scss" scoped></style>
