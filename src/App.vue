<template>
  <n-config-provider :theme="darkTheme">
    <n-global-style />
    <NMessageProvider>
      <Root />
    </NMessageProvider>
  </n-config-provider>
</template>

<script lang="ts" setup>
import { NConfigProvider, NGlobalStyle, darkTheme, NMessageProvider } from "naive-ui";

import Root from "./components/Root.vue";


</script>