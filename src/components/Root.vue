<template>
    <n-layout has-sider>
        <n-layout-sider content-style="padding: 24px;">
            <n-tabs type="line" animated>
                <n-tab-pane name="drawing" tab="Drawing">
                    <n-space vertical>
                        <n-button @click="stars = []" type="warning">Clear</n-button>
                        <n-dropdown size="small" trigger="click" :options="currentStarOptions"
                            @select="(selected) => currentStar = selected">
                            <n-button>Select Star</n-button>
                        </n-dropdown>
                    </n-space>
                </n-tab-pane>
                <n-tab-pane name="external" tab="Save/Load">
                    <n-space vertical>
                        <n-input v-model:value="jwt" placeholder="JWT token" />
                        <n-button @click="load_stars_clicked">Load Stars From Disk</n-button>
                        <n-button @click="save_stars_clicked">Save stars</n-button>
                        <n-button @click="send_to_arcade_clicked()">Send to arcade</n-button>
                    </n-space>
                </n-tab-pane>
            </n-tabs>
        </n-layout-sider>
        <n-layout>
            <n-layout-content>
                <StarRendering v-model:stars="stars" :currentStar="currentStar" />
            </n-layout-content>
        </n-layout>
    </n-layout>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { Star, load_stars, save_stars, send_stars } from "../rust-wrapper";

import { NDropdown, NTabs, NTabPane, NInput, NButton, NLayout, NLayoutSider, NLayoutContent, useMessage, NSpace } from "naive-ui";

import StarRendering from "./StarRender.vue";


const stars = ref<Star[]>([]);
const jwt = ref("");
const currentStar = ref(0);

const currentStarOptions = [{ label: "Random", key: 0 }].concat(Array(24).fill(0).map((element, index) => ({
    label: (index + 1).toString(),
    key: index + 1
})));
console.log(currentStarOptions);

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