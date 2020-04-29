<template>
  <mavon-editor v-model="value" :boxShadow="boxShadow" />
</template>

<script>
export default {
  props: ["value", "subfield"],
  boxShadow: false
};
</script>