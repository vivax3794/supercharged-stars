<template>
  <n-config-provider :theme="darkTheme">
    <n-global-style />
    <n-message-provider>
      <Root />
    </n-message-provider>
  </n-config-provider>
</template>

<script lang="ts" setup>
import { NConfigProvider, NGlobalStyle, darkTheme, NMessageProvider } from "naive-ui";

import Root from "./components/Root.vue";


</script>