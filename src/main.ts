
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

import { createApp } from 'vue'
import App from './App.vue'

try {
    const { shouldUpdate, manifest } = await checkUpdate()
    if (shouldUpdate) {
        // display dialog
        await installUpdate()
        // install complete, restart app
        await relaunch()
    }
} catch (error) {
    console.log(error)
}


createApp(App).mount('#app')
