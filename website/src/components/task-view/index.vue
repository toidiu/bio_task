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

    <div class="bg">
      <template>
        <task-view
          v-if="portState != null"
          :port-state="portState"
          @calc-investment-event="calcInvestmentHandler"
        />
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import TaskView from "./TaskView.vue";
import router from "../../index.js";
import { Task } from "./models";
import { Ticker, Action } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  components: {
    NavView,
    ErrorsView,
    LoaderView,
    TaskView
  },
  data() {
    return {
      portState: null, //TaskResp
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
      this.portState = {
        tasks: [
          { itemId: 1, projectId: 1, title: "asdf", description: "de" },
          {
            itemId: 2,
            projectId: 1,
            title: "asdfasd",
            description: "de asdfsdf"
          }
        ]
      };
      //this.$appGlobal.axi
      //  .get(`portfolio/actual/${this.actualId}`)
      //  .then(resp => {
      //    this.portState = resp.data;
      //    this.isLoading = false;
      //  })
      //  .catch(error => {
      //    this.errors.push(error.response);
      //    this.isLoading = false;
      //  });
      this.isLoading = false;
    },
    calcInvestmentHandler(amount: Number) {
      this.clearErrors();
      this.buyNextState = null;
      this.isLoading = true;
      // FIXME ==========================
      this.$appGlobal.axi
        .get(
          `portfolio/actual/buy?goal_port_id=${
            this.portState.goal_id
          }&actual_port_id=${this.actualId}&amount=${amount}`
        )
        .then(resp => {
          this.isLoading = false;
          var actions = resp.data.actions;
          if (!Array.isArray(actions) || !actions.length) {
            this.errors.push(
              "enter a higher amount to invest; unable to buy anything at this price"
            );
            return;
          }
          this.buyNextState = resp.data;
        })
        .catch(error => {
          this.errors.push(error.response);
          this.isLoading = false;
        });
    },
    clearBuyNext() {
      this.buyNextState = null;
    },
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>
