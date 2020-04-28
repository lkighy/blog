<template>
  <div class="book">
    <router-link to="/">
      Home
    </router-link> | <router-link to="/medium">
      Medium
    </router-link>
    <router-view />
    <img src="@/static/DSCF0866.jpg" alt="">
  </div>
</template>

<style>
.book {
  margin: auto;
}
</style>
