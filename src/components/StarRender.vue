<template>
    <canvas @mousemove="mouse_moved" @mousedown="canvas_down" @mouseup="canvas_up" ref="canvas" width="1000"
        height="510" />
</template>

<script lang="ts" setup>
import { ref, watchEffect, onMounted } from 'vue';
import { Star, COLORS } from '../rust-wrapper';

const canvas = ref<HTMLCanvasElement>();
const context = ref<CanvasRenderingContext2D | null>(null);

const props = defineProps<{
    stars: Star[],
}>();
const emit = defineEmits<{
    (e: "can_moved", mouse_down: boolean, x: number, y: number): void
    (e: "can_down", x: number, y: number): void
    (e: "can_up", x: number, y: number): void
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
        ctx.fillStyle = COLORS[star.currentStar !== 0 ? star.currentStar : Math.floor(Math.random() * 25)];
        ctx.fillRect(star.x, star.y, 5, 5);
    })

})

function get_location(event: MouseEvent, element: HTMLElement): [number, number] {
    let x = event.pageX;
    let y = event.pageY;

    while (element !== null) {
        x -= element.offsetLeft;
        y -= element.offsetTop;
        // @ts-ignore
        element = element.offsetParent;
    }

    return [x, y];
}

function mouse_moved(event: MouseEvent): void {
    if (canvas.value === undefined) return;
    if (event.target !== canvas.value) return;

    event.preventDefault();

    let [x, y] = get_location(event, canvas.value);

    emit("can_moved", event.buttons === 1, x, y);
}

function canvas_down(event: MouseEvent): void {
    if (canvas.value === undefined) return;
    if (event.target !== canvas.value) return;


    event.preventDefault();

    let [x, y] = get_location(event, canvas.value);

    emit("can_down", x, y);
}

function canvas_up(event: MouseEvent): void {
    if (canvas.value === undefined) return;
    if (event.target !== canvas.value) return;

    event.preventDefault();

    let [x, y] = get_location(event, canvas.value);

    emit("can_up", x, y);
}


</script>

<style lang="sass" scoped>
</style>