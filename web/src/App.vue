<template>
  <div class="book">
    <router-link to="/">
      Home
    </router-link> | <router-link to="/medium">
      Medium
    </router-link>
    <router-view />
    <img src="@/static/DSCF0866.jpg" alt="">
    <!-- <input type="text" v-model="markdownBody"> -->
    <textarea name="" id="" cols="30" rows="10" v-model="markdownBody" ></textarea>
    <div class="container" v-html="markdown"></div>
  </div>
</template>
<script>
export default {
  data() {
    return {
      markdownBody: '```rust\r\nprintln!("{}")```',
      markdown: '',
    }
  },
  mounted() {
    this.markdown = this.$md.render(this.markdownBody);
    // console.log("hello")
    // console.log(this.$md)
  },
  methods: {
    inputMd(e) {
    }
  },
  watch: {
    markdownBody(val) {
      this.markdown = this.$md.render(val);
    }
  }
}
</script>

<style>
.book {
  margin: auto;
}
</style>
