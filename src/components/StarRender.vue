<template>
    <div id="background">
        <div class="star" v-for="{ x, y, color } in stars" :style="{
            top: y + 'px',
            left: x + 'px',
            backgroundColor: COLORS[color] ?? '#f73693',
        }" />
    </div>
</template>

<script lang="ts" setup>
import { Star, COLORS } from '../rust-wrapper';

defineProps<{
    stars: Star[]
}>();

</script>

<style lang="sass" scoped>
#background
    position: relative
    background-color: black
    width: 1005px
    height: 515px

.star
    position: absolute
    width: 5px
    height: 5px
</style>