<template>
  <nav
    class="navbar level is-mobile"
    role="navigation"
    aria-label="main navigation"
  >
    <div class="navbar-brand ">
      <a class="navbar-item" v-on:click="goToHome">
        <img
          src="./../../static/images/biotask-logo.png"
          width="112"
          height="28"
        />
      </a>
    </div>

    <div class="level-right">
      <!-- <router-link to="/"> -->
      <!--   <a class="navbar-item">home</a> -->
      <!-- </router-link> -->
    </div>
  </nav>
</template>

<script lang="ts">
import Vue from "vue";
import router from "../index.js";

export default Vue.extend({
  props: {
    isUserAuth: {
      type: Boolean,
      default: false
    }
  },
  mounted() {
    this.toggleNav();
  },
  methods: {
    toggleNav: function() {
      // Get all "navbar-burger" elements
      const $navbarBurgers = Array.prototype.slice.call(
        document.querySelectorAll(".navbar-burger"),
        0
      );

      // Check if there are any navbar burgers
      if ($navbarBurgers.length > 0) {
        // Add a click event on each of them
        $navbarBurgers.forEach(el => {
          el.addEventListener("click", () => {
            // Get the target from the "data-target" attribute
            const target = el.dataset.target;
            const $target = document.getElementById(target);

            // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
            el.classList.toggle("is-active");
            $target.classList.toggle("is-active");
          });
        });
      }
    },
    logout: function(event) {
      event.preventDefault();
      this.$appGlobal.axi.post("users/logout").catch(error => {});
      router.push({ name: "login" });
    },
    goToHome() {
      if (this.$router.currentRoute.name == "home") {
        this.$router.go(0);
      } else {
        router.push({ name: "dash" });
      }
    }
  }
});
</script>

<style lang="scss" scoped></style>
