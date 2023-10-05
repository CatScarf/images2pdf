<script setup lang="ts">
import { onMounted, ref } from "vue";
import { readDir } from '@tauri-apps/api/fs'
import { invoke } from "@tauri-apps/api/tauri";

onMounted(async () => {
  const { listen } = await import('@tauri-apps/api/event')
  listen('tauri://file-drop', async (event: any) => {
    event.payload.forEach(read_path);
  })
})

const files = ref<string[]>([]);
const output = ref<string>("");
const running = ref<boolean>(false);
const success = ref<boolean>(false);
const quality = ref<string>("60");

const autoClear = ref<boolean>(true);
const autoConvert = ref<boolean>(true);

function isImageFormat(filename: string): boolean {
  const imageExtensions = ['.jpg', '.jpeg', '.png', '.bmp', '.gif', '.webp'];
  const ext = filename.slice(((filename.lastIndexOf(".") - 1) >>> 0) + 2).toLowerCase();
  return imageExtensions.includes('.' + ext);
}

function addToFiles(path: string) {
  if (isImageFormat(path)) {
    files.value.push(path);
  }
}

function get_output(path: string) {
  const path_list = path.split(/[\/\\]/);
  const dir = path_list.slice(0, -1).join('/');
  const name = path_list.slice(-1)[0].split('.')[0];
  return dir + '/' + name + '.pdf';
}

const zoneBouncing = ref(false);

async function read_path(path: string) {
  console.log(path);
  if (autoClear.value) {
    clear();
  }
  const files_before = files.value.length;
  try {
    const dirContent = await readDir(path);
    for (let file of dirContent) {
      let path: any = file;
      try {
        await readDir(path);
      } catch (error) {
        addToFiles(file.path!);
      }
    }
  } catch (error) {
    addToFiles(path);
  }
  if (files.value.length == files_before) {
    zoneBouncing.value = true;
    setTimeout(() => zoneBouncing.value = false, 300);
  } else {
    output.value = get_output(path);
    success.value = false;
    if (autoConvert.value) {
      convert();
    }
  }
}

function file_name(path: string) {
  return path.split('/').pop()?.split('\\').pop();
}

function clear() {
  files.value = [];
  output.value = "";
  success.value = false;
}

const convertBouncing = ref(false);
let convertBouncingStart = 0;

async function convert() {
  if (running.value || files.value.length == 0) {
    convertBouncing.value = false;
    convertBouncing.value = true;
    convertBouncingStart = Date.now();
    setTimeout(() => {
      if (Date.now() - convertBouncingStart >= 200) {
        convertBouncing.value = false;
      }
    }, 300);
    return;
  }

  running.value = true;
  success.value = false;
  new Promise(async (_, reject) => {
    try {
      const result = await invoke("convert", {
        paths: files.value,
        output: output.value,
        quality: Number(quality.value),
      });
      console.log(result);
      success.value = true;
    } catch (error) {
      console.error(error);
      reject(error);
    } finally {
      running.value = false;
    }
  })
}

</script>

<template>
  <div class="horizon">
    <div class="card" :class="{'bounce': zoneBouncing }">
      <div class="dropzone" :class="{ 'big-success': success }">
        <div v-show="files.length == 0" class="hint">
          Drag and Drop<br>Folder Here
        </div>

        <div class="file-list">
          <div class="file-name" v-for="file in files" :key="file">
            {{ file_name(file) }}
          </div>
        </div>

        <div v-if="success" class="success-outer absolute-center">
          <div class="check-mark absolute-center">✓</div>
          <div class="big-circle absolute-center">◯</div>
        </div>

      </div>
    </div>
    
    <div class="vertical">

      <div class="option">
        <div class="horizon option-title">
          <div>Auto Clear</div>
          <input class="checkbox" type="checkbox" v-model="autoClear">
        </div>
      </div>

      <button class="option button horizon" @click="clear">
        <div class="icon">↺</div><div>Clear</div>
      </button>

      
      <div class="option">
        <div class="horizon option-title">
          <div>Auto Convert</div>
          <input class="checkbox" type="checkbox" v-model="autoConvert">
        </div>
      </div>

      <button class="option button horizon" :class="{ 'running': running, 'bounce': convertBouncing }" @click="convert">
        <div class="icon">⇨</div><div>Convert</div>
      </button>
      
      <div class="option output" :class="{'success': success }">
        <div class="output-path">{{ output }}</div>
      </div>

      <div class="option" style="visibility: hidden">
        <div class="horizon option-title">
          <div>placehoder</div>
        </div>
      </div>

      <div class="option">
        <div class="horizon option-title">
          <div>Jpeg Quality</div>
          <div> {{ quality }} </div>
        </div>
        <div class="vertical">
          <input class="quality" type="range" min="0" max="100" step="10" v-model="quality">
        </div>
      </div>

    </div>

  </div>
</template>

<style>

div {
  user-select: none;
  -webkit-user-select: none;
}

:root {
  --base: min(1vw, 1vh);
}

.horizon {
  display: flex;
}

.vertical {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.dropzone {
  position: relative;
  width: calc(60 * var(--base));
  height: calc(80 * var(--base));
}

.card {
  background-color: white;
  box-shadow: .2rem .2rem .5rem .2rem rgba(0, 0, 0, 0.1);
  border-radius: 1rem;
  margin: .3rem;
  overflow: hidden;
}

.file-list {
  display: flex;
  flex-direction: column;
  font-size: .5rem;
  white-space: nowrap;
  overflow: auto;
  height: 100%;
  width: 100%;
  padding: .4rem 0;
}

.file-name {
  padding: 0 .6rem;
}

.button {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  font-size: .6rem;
  text-align: left;
  line-height: 1.4rem;
  box-shadow: .2rem .2rem .5rem .2rem rgba(0, 0, 0, 0.1);
}

.icon {
  width: 1rem;
  text-align: left;
}

.output {
  font-size: .6rem;
  min-height: 1.7rem;
  overflow: auto;
}

.output-path {
  padding: .1rem .6rem;
}

.running {
  background: linear-gradient(270deg, transparent, rgba(0, 128, 0, 0.526), transparent);
  background-size: 200% 100%;
  animation: running-gradient 2s linear infinite;
}

@keyframes running-gradient {
  0% {
    background-position: 100% 0;
  }
  100% {
    background-position: -100% 0;
  }
}

.success {
  animation: colorFadeIn 1s forwards;
}

@keyframes colorFadeIn {
  from { background-color: rgba(0, 0, 0, 0); }
  to { background-color: rgba(173, 205, 170, 0.422); }
}

.bounce {
  animation: bounce .2s ease infinite;
}

@keyframes bounce {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-.2rem); }
  75% { transform: translateX(.2rem); }
}

.hint {
  font-size: calc(6 * var(--base));
  line-height: calc(6 * var(--base));
  font-weight: bold;
  white-space: nowrap;

  text-align: center;
  color: gainsboro;
  position: relative;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.absolute-center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.success-outer {
  width: 100%;
  height: 100%;
  background-color: rgba(173, 205, 170, 0.422);
  color: rgb(94, 129, 88);
  pointer-events: none;

  animation: fadeIn 1s forwards;
}

.check-mark {
  font-size: calc(9 * var(--base));
}

.big-circle {
  font-size: calc(12 * var(--base))
}


.option {
  width: 8rem;
  font-size: .6rem;
  border-radius: 1rem;
  margin: .3rem;
}

.option-title {
  justify-content: space-between;
  padding: 0 .3rem;
}

.quality {
  box-shadow: none;
  width: 94%;
  padding: 0;
  margin: 0;
}

.checkbox {
  box-shadow: none;
  padding: 0;
  margin: 0;
}

</style>
