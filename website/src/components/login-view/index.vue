<template>
  <div>
    <template>
      <nav-view :is-user-auth="true" />
    </template>

    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <center-view>
      <div class="welcome">Welcome to Bio Task</div>

      <div>
        <form @submit.prevent="login">
          <div class="input-text">
            <label for="password">Password</label>
            <input
              class="input"
              id="password"
              type="password"
              name="password"
              placeholder="password"
              required
            />
          </div>
          <div class="login-button input-text">
            <button
              v-bind:class="{ 'is-loading': tryingLogin }"
              class="button is-primary"
              type="submit"
            >
              Enter
            </button>
          </div>
        </form>
      </div>
    </center-view>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import CenterView from "../CenterView.vue";
import PulseLoader from "vue-spinner/src/PulseLoader.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import router from "../../index.js";

import Vue from "vue";
import { EventEmitter } from "events";

export default Vue.extend({
  components: {
    NavView,
    CenterView,
    LoaderView,
    ErrorsView,
    PulseLoader
  },
  data() {
    return {
      errors: [] as String[],
      tryingLogin: false
    };
  },
  methods: {
    login(submitEvent) {
      this.clearErrors();
      var pw = submitEvent.target.elements.password.value;
      this.tryingLogin = true;
      this.$appGlobal.axi
        .post("users/login", {
          password: pw
        })
        .then(resp => {
          this.tryingLogin = false;
          router.push({ name: "home" });
        })
        .catch(error => {
          this.tryingLogin = false;
          var status = error.response.status;
          switch (status) {
            case 401: {
              this.errors.push("invalid login");
              return;
            }
            default:
              this.errors.push(`error ${status}. please try later`);
              return;
          }
        });
    },
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>

<style lang="scss" scoped>
nav {
}
input {
  max-width: 200px;
}
.welcome {
  padding: 20px;
  font-size: 30px;
}
.input-text {
  margin: 10px 0px;
  & label {
    display: block;
    margin-bottom: 5px;
  }
}
.login-button {
  padding-top: 20px;
  margin-bottom: 30px;
}
</style>
