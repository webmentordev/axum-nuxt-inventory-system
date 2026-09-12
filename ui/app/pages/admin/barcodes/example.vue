<template>
    <svg v-if="type === 'code128'" viewBox="0 0 120 40" class="w-full h-full">
        <rect v-for="(w, i) in code128Bars" :key="i" :x="code128X(i)" y="2" :width="w" height="30" fill="#111" />
    </svg>

    <svg v-else-if="type === 'ean13'" viewBox="0 0 120 46" class="w-full h-full">
        <rect x="2" y="2" width="2" height="30" fill="#111" />
        <rect x="6" y="2" width="1" height="30" fill="#111" />
        <rect v-for="(w, i) in ean13Bars" :key="i" :x="ean13X(i)" y="2" :width="w" height="26" fill="#111" />
        <rect x="58" y="2" width="1" height="30" fill="#111" />
        <rect x="61" y="2" width="1" height="30" fill="#111" />
        <rect x="112" y="2" width="1" height="30" fill="#111" />
        <rect x="116" y="2" width="2" height="30" fill="#111" />
        <text x="60" y="43" text-anchor="middle" font-size="8" fill="#111" font-family="monospace">6 901234
            567892</text>
    </svg>

    <svg v-else-if="type === 'upc_a'" viewBox="0 0 120 46" class="w-full h-full">
        <rect x="6" y="2" width="1" height="30" fill="#111" />
        <rect x="10" y="2" width="1" height="30" fill="#111" />
        <rect v-for="(w, i) in upcaBars" :key="i" :x="upcaX(i)" y="2" :width="w" height="26" fill="#111" />
        <rect x="58" y="2" width="1" height="30" fill="#111" />
        <rect x="61" y="2" width="1" height="30" fill="#111" />
        <rect x="112" y="2" width="1" height="30" fill="#111" />
        <rect x="116" y="2" width="1" height="30" fill="#111" />
        <text x="60" y="43" text-anchor="middle" font-size="8" fill="#111" font-family="monospace">036000 291452</text>
    </svg>

    <svg v-else viewBox="0 0 40 40" class="h-full">
        <rect x="2" y="2" width="12" height="12" fill="none" stroke="#111" stroke-width="2" />
        <rect x="5" y="5" width="6" height="6" fill="#111" />
        <rect x="26" y="2" width="12" height="12" fill="none" stroke="#111" stroke-width="2" />
        <rect x="29" y="5" width="6" height="6" fill="#111" />
        <rect x="2" y="26" width="12" height="12" fill="none" stroke="#111" stroke-width="2" />
        <rect x="5" y="29" width="6" height="6" fill="#111" />
        <rect x="18" y="2" width="3" height="3" fill="#111" />
        <rect x="22" y="6" width="3" height="3" fill="#111" />
        <rect x="18" y="10" width="3" height="3" fill="#111" />
        <rect x="26" y="18" width="3" height="3" fill="#111" />
        <rect x="18" y="18" width="3" height="3" fill="#111" />
        <rect x="34" y="18" width="3" height="3" fill="#111" />
        <rect x="30" y="22" width="3" height="3" fill="#111" />
        <rect x="18" y="26" width="3" height="3" fill="#111" />
        <rect x="22" y="30" width="3" height="3" fill="#111" />
        <rect x="18" y="34" width="3" height="3" fill="#111" />
        <rect x="26" y="30" width="3" height="3" fill="#111" />
        <rect x="34" y="34" width="3" height="3" fill="#111" />
    </svg>
</template>

<script setup lang="js">
const props = defineProps({
    type: { type: String, required: true }
});

const code128Widths = [3, 1, 1, 2, 1, 3, 1, 1, 2, 4, 1, 1, 3, 2, 1, 1, 2, 3, 1, 2, 1, 1, 4, 1, 2, 1, 3, 1, 1, 2];
const code128Bars = computed(() => code128Widths);
function code128X(i) {
    let x = 2;
    for (let j = 0; j < i; j++) x += code128Widths[j] + 1;
    return x;
}

const ean13Widths = [2, 1, 3, 1, 1, 2, 2, 1, 1, 3, 1, 2, 2, 3, 1, 1, 2, 1, 1, 3, 2, 1, 2, 2, 1, 1, 3, 1, 2, 1, 2, 2, 1, 1, 2, 3, 1, 1, 2, 1, 2, 1, 3, 1, 1];
const ean13Bars = computed(() => ean13Widths);
function ean13X(i) {
    let x = 9;
    for (let j = 0; j < i; j++) x += ean13Widths[j];
    return x;
}

const upcaWidths = [1, 2, 2, 1, 2, 1, 1, 1, 2, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2, 2, 1, 1];
const upcaBars = computed(() => upcaWidths);
function upcaX(i) {
    let x = 13;
    for (let j = 0; j < i; j++) x += upcaWidths[j];
    return x;
}
</script>