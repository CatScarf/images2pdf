<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Convert from "./components/Convert.vue";
</script>

<template>
  <div>
    <Convert class="convert"/>
  </div>
</template>

<style scoped>
.convert {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>
