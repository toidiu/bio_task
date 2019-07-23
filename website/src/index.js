import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import Login from "./components/login-view/index.vue";
import Task from "./components/task-view/index.vue";
import PageNotFound from "./components/page-not-found/index.vue";
import "./../node_modules/bulma/css/bulma.css";
import Config from "../config";
import axios from "axios";

Vue.use(VueRouter);

const router = new VueRouter({
  routes: [
    {
      path: "/",
      component: Login
    },
    {
      path: "/login",
      component: Login,
      name: "login"
    },
    {
      path: "/tasks",
      component: Task,
      name: "home"
    },
    {
      path: "/project",
      component: Task,
      name: "project"
    },
    {
      path: "*",
      component: PageNotFound,
      name: "notfound"
    }
  ]
});

const ax = axios.create({
  baseURL: `http://${Config.api.host}:${Config.api.port}/`,
  timeout: Config.api.timeoutMs,
  withCredentials: true
  //headers: { "Access-Control-Max-Age": "1" },
});

ax.interceptors.response.use(
  function(response) {
    return response;
  },
  function(error) {
    if (error.response == undefined) {
      console.log("THIS MIGHT BE CORS OR UNKNOWN STUFF");
    } else if (401 === error.response.status) {
      router.push({ name: "login" });
      return Promise.reject(error);
      // } else if (404 === error.response.status) {
      //   // FIXME ==========================
      //   router.push({ name: "dash" });
      //   return Promise.reject(error);
    }
  }
);

const shared = {
  config: Config,
  axi: ax
};

shared.install = function() {
  Object.defineProperty(Vue.prototype, "$appGlobal", {
    get() {
      return shared;
    }
  });
};
Vue.use(shared);

const app = new Vue({
  router,
  render: createEle => createEle(App)
}).$mount("#app");

export default router;
