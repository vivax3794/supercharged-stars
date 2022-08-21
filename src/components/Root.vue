<template>
    <n-layout has-sider>
        <n-layout-sider content-style="padding: 24px;">
            <n-tabs type="line" animated>
                <n-tab-pane name="drawing" tab="Drawing">
                    <n-space vertical>
                        <span :style="{ color: starStore.length < 9000 ? 'white' : 'red' }">{{ starStore.length }} /
                            9000</span>
                        <n-button @click="starStore.data = []" type="warning">Clear</n-button>
                        <n-dropdown size="small" trigger="click" :options="currentStarOptions"
                            @select="(selected) => currentStar = selected">
                            <n-button>Select Star</n-button>
                        </n-dropdown>
                        Symmetry amount:
                        <n-slider v-model:value="symmetry_amount" :min="1" :max="10" :step="1" />
                        <n-button @click="y_level_color">Rainbow colors!</n-button>
                    </n-space>
                </n-tab-pane>
                <n-tab-pane name="external" tab="Save/Load">
                    <n-space vertical>
                        <n-input v-model:value="jwt" placeholder="JWT token" />
                        <n-button @click="load_stars_clicked">Load Stars From Disk</n-button>
                        <n-button @click="save_stars_clicked">Save stars</n-button>
                        <n-button @click="send_to_arcade_clicked">Send to arcade</n-button>
                        <n-button @click="load_image_clicked">Load Image</n-button>
                    </n-space>
                </n-tab-pane>
            </n-tabs>
        </n-layout-sider>
        <n-layout>
            <n-layout-content>
                <StarRendering @can_moved="draw" />
            </n-layout-content>
        </n-layout>
    </n-layout>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { load_stars, save_stars, send_stars, load_image_colors } from "../rust-wrapper";

import { NSlider, NDropdown, NTabs, NTabPane, NInput, NButton, NLayout, NLayoutSider, NLayoutContent, useMessage, NSpace } from "naive-ui";

import StarRendering from "./StarRender.vue";
import { useStarsStore } from "../stores/stars";

const starStore = useStarsStore();
const symmetry_amount = ref(1);

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
            starStore.data = new_stars
            if (note !== null) message.info(note)
        })
        .catch(error_happend);
}

function save_stars_clicked(): void {
    save_stars(starStore.data)
        .then(() => message.success("stars saved to disk"))
        .catch(error_happend);
}

function send_to_arcade_clicked(): void {
    send_stars(jwt.value, starStore.data)
        .then(() => message.success("sent stars to matisse!"))
        .catch(error_happend);
}

function load_image_clicked(): void {
    load_image_colors()
        .then((new_stars) => {
            console.log(new_stars);
            starStore.data = new_stars
        })
        .catch(error_happend);
}

function draw(mouse_down: boolean, org_x: number, org_y: number): void {
    if (mouse_down) {
        create_symmetry(org_x, org_y).forEach(([x, y]) => {
            starStore.data.push({ x: x, y: y, currentStar: currentStar.value });
        })
    }

}

function create_symmetry(x: number, y: number): [number, number][] {
    var ctrX = 500;
    var ctrY = 250;
    var relX = x - ctrX;
    var relY = ctrY - y;
    var dist = Math.hypot(relX, relY);
    var angle = Math.atan2(relX, relY);  // Radians
    var result: [number, number][] = [];
    for (var i = 0; i < symmetry_amount.value; i++) {
        var theta = angle + Math.PI * 2 / symmetry_amount.value * i;  // Radians
        x = ctrX + Math.sin(theta) * dist;
        y = ctrY - Math.cos(theta) * dist;
        if (0 <= x && x <= 1000 && 0 <= y && y <= 500) {
            result.push([x, y]);
        }

    }
    return result;
}

function y_level_color(): void {
    starStore.data = starStore.data.map((value) => ({ x: value.x, y: value.y, currentStar: Math.floor(value.y / 500 * 24) + 1 }));
}
</script>