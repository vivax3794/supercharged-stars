<template>
    <canvas ref="canvas" width="1000" height="510" />
</template>

<script lang="ts" setup>
import { ref, watchEffect, onMounted } from 'vue';
import { Star, COLORS } from '../rust-wrapper';

const canvas = ref<HTMLCanvasElement>();
const context = ref<CanvasRenderingContext2D | null>(null);

const props = defineProps<{
    stars: Star[]
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


</script>

<style lang="sass" scoped>
</style>