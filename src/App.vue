<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Convert from "./components/Convert.vue";
import { open } from "@tauri-apps/api/shell";

function openLink(link: string) {
  console.log(`open ${link}`);
  open(link);
}

</script>

<template>
  <Convert class="convert"/>
  <div class="contact-us">
    <div>
      My
    </div>
    <button class="contact-btn github" @click="openLink('https://github.com/CatScarf/images2pdf')">
      Github
    </button>
    <button class="contact-btn bilibili" @click="openLink('https://space.bilibili.com/6865778')">
      Bilibili
    </button>
  </div>
</template>

<style scoped>
.convert {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.flex-col {
  display: flex;
  flex-direction: column;
}
.contact-us {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);

  opacity: 0.3;

  margin: .6rem 0;
  justify-content: center;
  align-items: center;
  gap: .6rem;
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  font-size: .6rem;
}

.contact-btn {
  color: rgb(255, 255, 255);
  border-radius: 1rem;
}

.github {
  background-color: black;
}

.bilibili {
  background-color: #ff6699;
}

</style>
