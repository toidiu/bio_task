<template>
  <div>
    <template>
      <loader-view class="" v-show="isLoading" :is-loading="isLoading" />
    </template>

    <template>
      <nav-view />
    </template>

    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <template>
      <task-view
        v-if="tasksState != null"
        :tasks-state="tasksState"
        @calc-investment-event="calcInvestmentHandler"
      />
    </template>

    <task-modal />
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import TaskView from "./TaskView.vue";
import TaskModal from "./TaskModal.vue";
import router from "../../index.js";
import { Task } from "./models";
import { Ticker, Action } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  components: {
    NavView,
    ErrorsView,
    LoaderView,
    TaskView,
    TaskModal
  },
  data() {
    return {
      tasksState: null, //TaskResp
      actualId: this.$route.params.id,
      isLoading: true,
      buyNextState: null, //BuyNextResp
      errors: [] as String[]
    };
  },
  mounted() {
    this.getPortfolio();
    console.log();
  },
  methods: {
    getPortfolio() {
      this.clearErrors();
      /* get portfolio */
      this.isLoading = true;
      this.$appGlobal.axi
        .get(`tasks/incomplete`)
        .then(resp => {
          this.tasksState = resp.data;
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.response);
          this.isLoading = false;
        });
      this.isLoading = false;
    },
    calcInvestmentHandler(amount: Number) {},
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>
