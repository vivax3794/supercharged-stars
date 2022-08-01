<template>
    <n-layout has-sider>
        <n-layout-sider content-style="padding: 24px;">
            <n-space>
                <n-input v-model:value="jwt" placeholder="JWT token" />
                <n-button @click="load_stars_clicked">Load Stars From Disk</n-button>
                <n-button @click="save_stars_clicked">Save stars</n-button>
                <n-button @click="send_to_arcade_clicked()">Send to arcade</n-button>
            </n-space>
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
import { Star, load_stars, save_stars, send_stars } from "../rust-wrapper";

import { NInput, NButton, NLayout, NLayoutSider, NLayoutContent, useMessage, NSpace } from "naive-ui";

import StarRendering from "./StarRender.vue";


const stars = ref<Star[]>([]);
const jwt = ref("");

const message = useMessage();

function error_happend(err: object): void {
    console.error(err);
    message.error(`${err}`);
}

function load_stars_clicked(): void {
    load_stars()
        .then(([new_stars, note]) => {
            console.log(new_stars);
            stars.value = new_stars
            if (note !== null) message.info(note)
        })
        .catch(error_happend);
}

function save_stars_clicked(): void {
    save_stars(stars.value)
        .then(() => message.success("stars saved to disk"))
        .catch(error_happend);
}

function send_to_arcade_clicked(): void {
    send_stars(jwt.value, stars.value)
        .then(() => message.success("sent stars to matisse!"))
        .catch(error_happend);
}

</script>