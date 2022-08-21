<template>
    <canvas @mousemove="mouse_moved" ref="canvas" width="1000" height="510" />
</template>

<script lang="ts" setup>
import { useStarsStore } from '../stores/stars';
import { ref, watchEffect, onMounted } from 'vue';
import { Star, COLORS } from '../rust-wrapper';

const starStore = useStarsStore()

const canvas = ref<HTMLCanvasElement>();
const context = ref<CanvasRenderingContext2D | null>(null);

const emit = defineEmits<{
    (e: "can_moved", mouse_down: boolean, x: number, y: number): void
}>();

onMounted(() => {
    context.value = canvas.value?.getContext("2d") ?? null;
})

watchEffect(() => {
    if (context.value === null) return;

    let ctx = context.value;

    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, 1000, 510);

    starStore.data.forEach((star) => {
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


</script>