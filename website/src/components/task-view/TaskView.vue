<template>
  <div>
    <div>debug: sort={{ currentSort }}, dir={{ currentSortDir }}</div>
    <table class="table">
      <tr>
        <template v-for="([colName, colKey], idx) in columns">
          <th @click="sort(colKey)">
            <div class="sort-wrapper">
              <img class="sort-img" src="./../../../static/images/sort.svg" />
              <div class="sort-text">
                {{ colName }}
              </div>
            </div>
          </th>
        </template>
      </tr>

      <template v-for="(task, tidx) in sortedTasksState">
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
    tasksState: Array
  },
  data: function() {
    return {
      columns: [
        ["Id", "itemId"],
        ["Title", "title"],
        ["Description", "description"],
        ["ProjectId", "projectId"],
        ["Deadline", "deadlineDate"],
        ["MemberId", "memberId"]
      ],
      currentSort: "deadlineDate",
      currentSortDir: "asc"
    };
  },
  methods: {
    sort: function(s) {
      //if s == current sort, reverse
      if (s === this.currentSort) {
        this.currentSortDir = this.currentSortDir === "asc" ? "desc" : "asc";
      }
      this.currentSort = s;
    }
  },
  computed: {
    sortedTasksState: function() {
      return this.tasksState.sort((a, b) => {
        let modifier = 1;
        if (this.currentSortDir === "desc") modifier = -1;
        if (a[this.currentSort] < b[this.currentSort]) return -1 * modifier;
        if (a[this.currentSort] > b[this.currentSort]) return 1 * modifier;
        return 0;
      });
    }
  }
});
</script>

<style lang="scss" scoped>
div {
  display: ruby;
}
.sort-img {
  width: 14px;
  height: auto;
  vertical-align: middle;
}
.sort-text {
}
table {
  table-layout: unset;
}
th,
td {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: normal;
  padding: 10px 10px;
  text-align: center;
  font-size: 13px;
  min-width: 40px;
  width: 45px;
  max-width: 400px;
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
