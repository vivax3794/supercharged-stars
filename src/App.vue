<template>
  <n-config-provider :theme="darkTheme">
    <n-global-style />
    <Root />
  </n-config-provider>
</template>

<script lang="ts" setup>
import { NConfigProvider, NGlobalStyle, darkTheme } from "naive-ui";
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

import Root from "./components/Root.vue";

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

</script>