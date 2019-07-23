<template>
  <div class="modal" :class="{ 'is-active': modalActive }">
    <div class="modal-background" v-on:click="closeModal"></div>
    <div class="modal-card">
      <div class="modal-card-head">
        <p class="modal-card-title">{{ modalTask }}</p>
        <button
          class="delete"
          aria-label="close"
          v-on:click="closeModal"
        ></button>
      </div>
      <section class="modal-card-body">
        body
      </section>
      <footer class="modal-card-foot">
        foot
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
  props: {},
  data: function() {
    return {
      columns: [
        //["Item Id", "itemId"],
        ["Title", "title"],
        ["Deadline", "deadlineDate"],
        ["Description", "description"],
        ["Status", "statusKey"],
        ["Member Name", "memberName"]
      ],
      currentSort: "deadlineDate",
      currentSortDir: "asc",
      max: 5,
      value: 3,
      pageSize: 25,
      currentPage: 1,
      totalPages: 0,
      modalActive: true,
      modalTask: {} as Object
    };
  },
  methods: {
    sort: function(s) {
      //if s == current sort, reverse
      if (s === this.currentSort) {
        this.currentSortDir = this.currentSortDir === "asc" ? "desc" : "asc";
      }
      this.currentSort = s;
    },
    closeModal: function() {
      this.modalActive = false;
    },
    editTask: function(t) {
      console.log(t);
    },
    nextPage: function() {
      if (this.currentPage * this.pageSize < this.tasksState.length)
        this.currentPage++;
    },
    prevPage: function() {
      if (this.currentPage > 1) this.currentPage--;
    },
    getPercent: function(v) {
      return (v / 5) * 100 + "%";
    },
    getSortImgUrl: function(s) {
      if (this.currentSort === s) {
        if (this.currentSortDir === "asc") {
          return require("./../../../static/images/sort-up.svg");
        } else if (this.currentSortDir === "desc") {
          return require("./../../../static/images/sort-down.svg");
        }
      } else {
        return require("./../../../static/images/sort.svg");
      }
    },
    getSortTextColor: function(s) {
      if (this.currentSort === s) {
        return "active";
      } else {
        return "";
      }
    }
  }
});
</script>

<style lang="scss" scoped></style>
