<template>
  <div>
    <mavon-editor v-model="value" :boxShadow="boxShadow" boxShadowStyle="0 2px 12px 0 rgba(0, 0, 0, 0)"/>
  </div>
</template>

<script>
export default {
  data() {
    return {
      value: ""
    };
  }
};
</script>

<style lang="scss" scoped>
</style>