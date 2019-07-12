<template>
  <div>
    <table class="table">
      <tr>
        <template v-for="([colName, colKey], idx) in columns">
          <th @click="sort(colKey)">
            <a class="sort-wrapper">
              <img class="sort-img" :src="getSortImgUrl(colKey)" />
              <div class="sort-text" :class="getSortTextColor(colKey)">
                {{ colName }}
              </div>
            </a>
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
        ["Deadline", "deadlineDate"],
        ["Title", "title"],
        ["Description", "description"],
        //["Project Title", "projectTitle"],
        ["Member Name", "memberName"]
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
    },
    getSortImgUrl: function(s) {
      console.log(s);
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
      console.log(s);
      if (this.currentSort === s) {
        return "active";
      } else {
        return "";
      }
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
.sort-wrapper {
  display: ruby;
}
.sort-img {
  width: 14px;
  height: auto;
  vertical-align: middle;
  margin-bottom: 2px;
}
.sort-text {
  color: black;
  font-size: 1.2em;
  &.active {
    color: #078484;
  }
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
