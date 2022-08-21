import { Star } from './../rust-wrapper';
import { defineStore } from "pinia"
import { ref, computed } from "vue";

export const useStarsStore = defineStore("stars", () => {
    const data = ref<Star[]>([]);

    const length = computed(() => data.value.length)

    return { data, length }

})