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
        {{ modalTask.description }}
      </section>
      <footer class="modal-card-foot">
        {{ modalTask.itemId }}
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
