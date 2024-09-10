import { createApp } from "vue";
import App from "./App.vue";
import "./assets/main.css";

import { invoke } from '@tauri-apps/api/core'
import { platform } from '@tauri-apps/plugin-os'

createApp(App).mount("#app");


// Ctrl+Q on Windows/Linux or Cmd+Q on macOS to quit the app
const platformName = platform();

function quit() {
    void invoke('quit', { code: 0 });
}

if (platformName === 'windows' || platformName === 'linux') {
    window.addEventListener('keydown', function (e) {
        if (e.ctrlKey && e.key === 'q') {
            quit();
        }
    });
} else if (platformName === 'macos') {
    window.addEventListener('keydown', function (e) {
        if (e.metaKey && e.key === 'q') {
            quit();
        }
    });
}
