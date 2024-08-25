import { createApp } from "vue";
import App from "./App.vue";
import "./assets/main.css";

import { exit } from '@tauri-apps/api/process'
import { platform } from '@tauri-apps/api/os'

createApp(App).mount("#app");


// Ctrl+Q on Windows/Linux or Cmd+Q on macOS to quit the app
platform().then(platformName => {
    if (platformName === 'win32' || platformName === 'linux') {
        window.addEventListener('keydown', function (e) {
            if (e.ctrlKey && e.key === 'q') {
                exit(0);
            }
        });
    } else if (platformName === 'darwin') {
        window.addEventListener('keydown', function (e) {
            if (e.metaKey && e.key === 'q') {
                exit(0);
            }
        });
    }
});
