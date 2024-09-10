<script setup>
import { ref, onMounted, watch } from 'vue'
import { copyTextToClipboard } from './clipboard'
import CopiedToast from './components/CopiedToast.vue'
import { invoke } from '@tauri-apps/api/tauri'

const MODE_SC2TC = 0;
const MODE_TC2SC = 1;

const mode = ref(MODE_SC2TC);
const phrase = ref(true);

function switchMode() {
  mode.value = mode.value === MODE_SC2TC ? MODE_TC2SC : MODE_SC2TC;
  inputText.value = output.value;
}


const inputText = ref('');
const output = ref('');

async function convert() {
  let func = mode.value === MODE_SC2TC ? 'sc2tc' : 'tc2sc';
  if(phrase.value) {
    func += '_phrase';
  }
  output.value = await invoke(func, { input: inputText.value });
}

watch(inputText, convert);
watch(phrase, convert);

const copiedToast = ref(null);

async function sendToClipboard() {
  if(inputText.value === '') {
    return;
  }
  const ok = await copyTextToClipboard(output.value);
  if(ok) {
    copiedToast.value.trigger();
  } else {
    alert('Failed to copy to clipboard. Please report this issue.');
  }
}

const inputArea = ref(null);

onMounted(function() {
  if (inputArea.value) {
    inputArea.value.focus();
  }

  // Enable ":active" pseudo-class on mobile safari. https://stackoverflow.com/q/3885018/
  if(/iPad|iPhone|iPod/.test(window.navigator.userAgent)) {
    const buttons = document.querySelectorAll('button');
    buttons.forEach(btn => {
      btn.addEventListener('touchstart', function() {}, { passive: false });
    });
  }
});

</script>

<template>
  <div class="bg-app text-app-blue min-h-screen flex flex-col">
    <nav class="theme-app flex justify-between text-white">
      <h1 class="flex items-center h-16 ml-4">
        <span class="text-4xl">CC Converter</span>
      </h1>
      <div class="flex">
        <a class="flex items-center font-medium p-2 mr-2 hover:bg-gray-900" href="https://github.com/qwinsi/cc-converter" target="_blank">
          <img class="inline h-9" src="./assets/github-mark-white.svg" alt="Github logo" />
          <span class="text-lg ml-2 mr-4">Open-source</span>
        </a>
      </div>
    </nav>
    <div class="text-center text-app-blue p-4">
      Convert between Traditional Chinese and Simplified Chinese.
    </div>

    <main class="flex-1 flex flex-col justify-between m-4 ml-6 mr-6 border border-gray-700 rounded-lg">
      <div class="flex justify-between p-2 border-b border-gray-700">
        <div class="flex-1 flex justify-between">
          <span class="text-app-blue p-2">{{ mode === MODE_SC2TC ? 'Simplified Chinese' : 'Traditional Chinese' }}</span>
          <div>
            <button class="text-app-light-black p-2 rounded-lg hover:bg-gray-300 active:bg-gray-400"
              v-on:click="inputText = ''">Clear</button>
          </div>
        </div>

        <button class="pl-1 pr-1 rounded-lg ml-3 mr-3 hover:bg-gray-300 active:bg-gray-400"
          v-on:click="switchMode">
          <img class="inline" src="./assets/swap-icon.svg" alt="Swap icon" />
        </button>

        <div class="flex-1 flex justify-between relative">
          <span class="flex-1 text-app-blue p-2">{{ mode === MODE_SC2TC ? 'Traditional Chinese' : 'Simplified Chinese' }}</span>
          <div class="flex-1 text-app-light-black p-2">
            <input type="checkbox" id="phrase" v-model="phrase"/>
            <label for="phrase">Phrase</label>
          </div>
          <button class="text-app-light-black p-2 rounded-lg hover:bg-gray-300 active:bg-gray-400"
            v-on:click="sendToClipboard">Copy</button>
          <CopiedToast ref="copiedToast" id="copiedToast" />
        </div>
      </div>

      <!-- flex-row for desktop, flex-col for mobile -->
      <div class="flex-1 flex md:flex-row flex-col">
        <!-- input area -->
        <div class="flex-1 flex flex-col border border-gray-700 min-h-[200px]">
          <textarea ref="inputArea" class="flex-1 text-app-light-black p-4" v-model="inputText"></textarea>
        </div>

        <!-- output area -->
        <div class="flex-1 flex flex-col border border-gray-700 min-h-[200px]">
          <div class="flex-1 flex flex-col" id="outArea">
            <div class="flex-1 text-app-light-black p-4"> {{ output }} </div>
          </div>
        </div>
      </div>

    </main>


    <!-- items-center (i.e. style="align-items:center") is for vertical centering -->
    <div ref="renderArea" class="flex items-center text-center text-app-light-black p-4">
    </div>

    <footer class="theme-app text-center p-4">
      <p class="text-white">Powered by <a href="https://crates.io/crates/zhconv"
          target="_blank">zhconv-rs</a></p>
    </footer>
  </div>
</template>

<style>
.bg-app {
  background-color: #eeeeee;
}

.theme-app {
  background-color: #1d6c86;
}

.theme-warning {
  background-color: #fdf8e4;
}

.text-app-blue {
  color: #463968;
}

.text-app-light-black {
  color: #333333;
}

#outArea a {
  font-weight: bold;
  text-decoration: underline !important;
}

#outArea a:link, #outArea a:visited {
  color: #0000EE !important;
}

#outArea a:link:active, #outArea a:visited:active {
  color: #FF0000 !important;
}


/* https://stackoverflow.com/questions/36260013/react-display-line-breaks-from-saved-textarea */
#outArea {
  white-space: pre-line;
}

#copiedToast {
  position: absolute;
  top: -55px;
  right: -4px;
}
</style>
