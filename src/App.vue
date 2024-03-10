
<script>
import QuickSearch from "./components/index.vue";

export default {
  components: {
    QuickSearch,
  },
};
</script>
<template>
  <v-app>
    <QuickSearch />
  </v-app>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
