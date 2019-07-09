<template>
  <div>
    <table class="table">
      <tr>
        <template v-for="([colName, colKey], idx) in columns">
          <th>{{ colName }}</th>
        </template>
      </tr>

      <template v-for="(task, tidx) in portState.tasks">
        <tr>
          <template v-for="([colName, colKey], idx) in columns">
            <td>
              {{ task[colKey] }}
            </td>
          </template>
        </tr>
      </template>
    </table>
  </div>
</template>

<script lang="ts">
import { TaskResp } from "./models";
import Vue from "vue";

export default Vue.extend({
  props: {
    portState: Object as () => TaskResp
  },
  data: function() {
    return {
      columns: [
        ["Id", "itemId"],
        ["ProjectId", "projectId"],
        ["Title", "title"],
        ["Description", "description"]
      ]
    };
  },
  methods: {
    ticker_state: function(ticker: Ticker) {
      var deviation = this.portState.deviation_percent;
      return this.high_or_low(
        ticker.goal_percent,
        ticker.actual_percent,
        deviation
      );
    },
    high_or_low: function(goal: number, actual: number, deviation: number) {
      var diff = goal - actual;
      if (diff > 0 && Math.abs(diff) > deviation) {
        return "high";
      } else if (diff < 0 && Math.abs(diff) > deviation) {
        return "low";
      } else {
        return "balance";
      }
    }
  }
});
</script>

<style lang="scss" scoped>
th,
td {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  padding: 10px 10px;
  text-align: center;
  font-size: 13px;
}

th {
  background-color: #c9c9c9;
}

td {
  background-color: #f9f9f9;

  &.goal_percent,
  &.symbol {
    font-weight: bold;
  }
  &.summary {
    background-color: #d5d5d5;
  }
  &.stock {
    background-color: #eee;
  }
  &.actual_percent {
    &.balance {
      color: black;
    }
    &.low {
      color: green;
    }
    &.high {
      color: red;
    }
  }

  // percent
  &.fee,
  &.actual_percent,
  &.goal_percent {
    &::after {
      content: "%";
    }
  }

  // dollar
  &.price,
  &.actual_value,
  &.total_value {
    &::before {
      content: "$";
    }
  }
}
</style>
