<template>
    <div class="w-full h-full flex items-center justify-center">
        <svg v-if="type !== 'qr'" ref="svgEl"></svg>
        <canvas v-else ref="canvasEl"></canvas>
    </div>
</template>

<script setup lang="js">
import JsBarcode from 'jsbarcode';
import QRCode from 'qrcode';

const props = defineProps({
    type: { type: String, required: true }
});

const svgEl = ref(null);
const canvasEl = ref(null);

const sampleValues = {
    code128: 'SKU-000123',
    ean13: '5901234123457',
    upc_a: '036000291452'
};

const formatMap = {
    code128: 'CODE128',
    ean13: 'EAN13',
    upc_a: 'UPC'
};

async function render() {
    await nextTick();
    if (props.type === 'qr') {
        if (canvasEl.value) {
            QRCode.toCanvas(canvasEl.value, 'PRODUCT-000123', { width: 56, margin: 0 });
        }
        return;
    }
    if (svgEl.value) {
        JsBarcode(svgEl.value, sampleValues[props.type], {
            format: formatMap[props.type],
            width: 1.4,
            height: 26,
            displayValue: true,
            fontSize: 9,
            margin: 0
        });
    }
}

onMounted(render);
watch(() => props.type, render);
</script>