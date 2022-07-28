<template>
    <n-layout has-sider>
        <n-layout-sider>
            <n-button @click="load_stars_clicked">Load Stars From Disk</n-button>
        </n-layout-sider>
        <n-layout>
            <n-layout-content>
                <StarRendering :stars="stars" />
            </n-layout-content>
        </n-layout>
    </n-layout>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { Star, load_stars } from "../rust-wrapper";

import { NButton, NLayout, NLayoutSider, NLayoutContent, useMessage } from "naive-ui";

import StarRendering from "./StarRender.vue";
import { listen } from '@tauri-apps/api/event'

const stars = ref<Star[]>([]);
const message = useMessage();

function load_stars_clicked() {
    load_stars()
        .then((new_stars) => stars.value = new_stars)
        .catch((error) => message.error(error));
}

</script>