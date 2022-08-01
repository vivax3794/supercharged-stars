<template>
    <canvas @mousemove="(event) => canvas_clicked(event)" ref="canvas" width="1000" height="510" />
</template>

<script lang="ts" setup>
import { ref, watchEffect, onMounted } from 'vue';
import { Star, COLORS } from '../rust-wrapper';

const canvas = ref<HTMLCanvasElement>();
const context = ref<CanvasRenderingContext2D | null>(null);

const props = defineProps<{
    stars: Star[],
    currentStar: number,
}>();
const emit = defineEmits<{
    (e: "update:stars", new_stars: Star[]): void
}>();

onMounted(() => {
    context.value = canvas.value?.getContext("2d") ?? null;
})

watchEffect(() => {
    if (context.value === null) return;

    let ctx = context.value;

    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, 1000, 510);

    props.stars.forEach((star) => {
        ctx.fillStyle = COLORS[star.currentStar];
        ctx.fillRect(star.x, star.y, 5, 5);
    })

})


function canvas_clicked(event: MouseEvent): void {
    if (canvas.value === undefined) return;
    if (event.buttons !== 1 || event.target !== canvas.value) return;

    event.preventDefault();

    console.log(event);

    let element: HTMLElement | null = canvas.value;
    let x = event.pageX;
    let y = event.pageY;

    while (element !== null) {
        x -= element.offsetLeft;
        y -= element.offsetTop;
        // @ts-ignore
        element = element.offsetParent;
    }

    emit("update:stars", props.stars.concat([{ x: x, y: y, currentStar: props.currentStar }]));
}


</script>

<style lang="sass" scoped>
</style>