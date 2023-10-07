<script setup lang="ts">
import { onMounted, ref } from "vue";
import { readDir } from '@tauri-apps/api/fs'
import { invoke } from "@tauri-apps/api/tauri";

let hasNewFile = false;

onMounted(async () => {
  const { listen } = await import('@tauri-apps/api/event');
  listen('tauri://file-drop', async (event: any) => {
    hasNewFile = false;
    try {

      // Auto clear
      if (autoClear.value && files.value.length > 0) {
        clear();
      }

      // Read path
      await Promise.all(event.payload.map(read_path));

      // Auto sort
      if (autoSort.value && files.value.length > 0) {
        sort(true);
      }

      // Auto convert
      if (hasNewFile) {
        success.value = false;
        if (autoConvert.value) {
          console.log('auto convert');
          convert();
        }
      } else {
        trigger('zone', 'bounce', 300);
      }

    } catch (error) {
      console.error(error);
    }
  })
})

const files = ref<string[]>([]);
const output = ref<string>("");
const running = ref<boolean>(false);
const success = ref<boolean>(false);
const quality = ref<string>("60");

const autoClear = ref<boolean>(true);
const autoConvert = ref<boolean>(true);
const autoSort = ref<boolean>(true);

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
  let name = "";
  name = path_list.slice(-1)[0].split('.')[0];
  return dir + '/' + name + '.pdf';
}

async function read_path(path: string) {
  console.log(path);
  try {
    const dirContent = await readDir(path);
    for (let file of dirContent) {
      let path: any = file;
      try {
        await readDir(path);
      } catch (error) {
        hasNewFile = true;
        addToFiles(file.path!);
      }
    }
  } catch (error) {
    hasNewFile = true;
    addToFiles(path);
  } finally {
    output.value = get_output(path);
  }
}


const animationStart: { [key: string]: number } = {};

/**
 * Applies an animation effect to the element with the specified ID.
 * @param {string} id - The ID of the element to which the animation effect should be applied.
 * @param {string} animation - The animation class to apply, defaults to 'running'.
 * @param {number} duration - The duration of the animation, defaults to 500 milliseconds.
 */

function trigger(id: string, animation='running', duration=500): void {
  const targetElement = document.getElementById(id);
  const key = `${id}-${animation}`;
  animationStart[key] = Date.now();
  if (targetElement) {
    targetElement.classList.remove(animation);
    targetElement.classList.add(animation);
    setTimeout(() => {
      if (Date.now() - animationStart[key] >= duration) {
        targetElement.classList.remove(animation);
      }
    }, duration);
  } else {
    console.warn(`Element with id "${id}" not found.`);
  }
}

function file_name(path: string) {
  return path.split('/').pop()?.split('\\').pop();
}

function clear() {
  if (files.value.length > 0) {
    trigger('clear', 'trigger');
  } else {
    trigger('clear', 'bounce', 300);
  }
  files.value = [];
  output.value = "";
  success.value = false;
}

function sort(force=false) {
  if (force) {
    trigger('sort', 'trigger');
    files.value.sort();
  } else if (files.value.length > 0) {
    const befor = JSON.stringify(files.value);
    files.value.sort();
    if (befor != JSON.stringify(files.value)) {
      trigger('sort', 'trigger');
    } else {
      trigger('sort', 'bounce', 300);
    }
  } else {
    trigger('sort', 'bounce', 300);
  }
  files.value.sort();
}

async function convert() {
  const startTime = Date.now();

  if (running.value || files.value.length == 0) {
    trigger('convert', 'bounce', 300);
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
      const duration = (Date.now() - startTime);
      console.log('duration: ' + (Date.now() - startTime) / 1000 + 's');
      if (duration < 500) {
        setTimeout(() => {running.value = false;}, 500 - duration)
      } else {
        running.value = false;
      }
    }
  })
}

</script>

<template>
  <div class="horizon">
    <div id="zone" class="card">
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
      <button id="clear" class="option button horizon" @click="clear">
        <div class="icon">↺</div><div>Clear</div>
      </button>

      <div class="option">
        <div class="horizon option-title">
          <div>Auto Sort</div>
          <input class="checkbox" type="checkbox" v-model="autoSort">
        </div>
      </div>
      <button id="sort" class="option button horizon" @click="sort(false)">
        <div class="icon">↺</div><div>Sort</div>
      </button>

      
      <div class="option">
        <div class="horizon option-title">
          <div>Auto Convert</div>
          <input class="checkbox" type="checkbox" v-model="autoConvert">
        </div>
      </div>
      <button id="convert" class="option button horizon" :class="{ 'running': running }" @click="convert">
        <div class="icon">⇨</div><div>Convert</div>
      </button>

      <div class="option">
        <div class="output-outer" :class="{'success': success }">
          <div class="output">
            <div class="output-path">{{ output }}</div>
          </div>
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
  width: calc(60 * var(--base));
  height: calc(80 * var(--base));
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

.output-outer {
  margin: .6rem 0 0 0;
  border-radius: 1rem;
  overflow: hidden;
}

.output {
  font-size: .6rem;
  min-height: 1.7rem;
  max-height: 4.2rem;
  word-break: break-all;
  overflow: auto;
}

.output-path {
  padding: .1rem .5rem;
  line-height: 1rem;
}

.running {
  background: linear-gradient(270deg, transparent, transparent, rgba(0, 128, 0, 0.526), transparent, transparent);
  background-size: 200% 100%;
  animation: running-gradient .5s linear infinite;
}

.trigger {
  background: linear-gradient(270deg, transparent, transparent, rgba(0, 128, 0, 0.526), transparent, transparent);
  background-size: 200% 100%;
  animation: running-gradient .5s linear forwards;
}

@keyframes running-gradient {
  0% {
    background-position: 150% 0;
  }
  100% {
    background-position: -50% 0;
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
  width: 120%;
  height: 120%;
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
  margin: 0 0 .1rem .4rem;
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
