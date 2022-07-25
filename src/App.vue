<template>
  <n-config-provider :theme="darkTheme">
    <n-global-style />
    <Root />
  </n-config-provider>
</template>

<script lang="ts" setup>
import { NConfigProvider, NGlobalStyle, darkTheme } from "naive-ui";

import Root from "./components/Root.vue";
</script>