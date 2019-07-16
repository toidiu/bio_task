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
              <div v-if="colKey === 'statusKey'">
                <div class="progress-wrapper">
                  <progress
                    class="progress is-primary"
                    :value="task[colKey]"
                    max="5"
                    >15%</progress
                  >
                  <p class="progress-value has-text-black">
                    {{ getPercent(task[colKey]) }}
                  </p>
                </div>
              </div>
              <div v-else>
                {{ task[colKey] }}
              </div>
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
        ["Title", "title"],
        ["Deadline", "deadlineDate"],
        ["Description", "description"],
        ["Status", "statusKey"],
        ["Member Name", "memberName"]
      ],
      currentSort: "deadlineDate",
      currentSortDir: "asc",
      max: 5,
      value: 3
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
    getPercent: function(v) {
      return (v / 5) * 100 + "%";
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
.progress-wrapper {
  position: relative;
}

.progress-value {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
  font-size: calc(1rem / 1.5);
  line-height: 1rem;
  font-weight: bold;
}

.progress.is-small + .progress-value {
  font-size: calc(0.75rem / 1.5);
  line-height: 0.75rem;
}

.progress.is-medium + .progress-value {
  font-size: calc(1.25rem / 1.5);
  line-height: 1.25rem;
}

.progress.is-large + .progress-value {
  font-size: calc(1.5rem / 1.5);
  line-height: 1.5rem;
}
.progress {
  border-radius: 2px;
}
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
