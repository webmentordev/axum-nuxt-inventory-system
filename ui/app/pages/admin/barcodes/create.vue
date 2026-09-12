<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Add barcodes</h1>
            <p class="text-sm text-zinc-500 mt-1">Scan barcodes to register them.</p>

            <div class="mt-4 border border-dark-300 rounded-lg bg-dark-100 p-4">
                <button type="button" @click="legendOpen = !legendOpen"
                    class="flex items-center justify-between w-full text-left">
                    <span class="text-sm font-semibold text-zinc-300">What does each type look like?</span>
                    <Icon :name="legendOpen ? 'mdi:chevron-up' : 'mdi:chevron-down'" size="18" class="text-zinc-500" />
                </button>

                <div v-if="legendOpen" class="grid grid-cols-2 gap-3 mt-3">
                    <div v-for="option in typeOptions" :key="option.value"
                        class="border border-dark-300 rounded-lg p-3">
                        <div class="bg-white rounded-md p-2 mb-2 flex items-center justify-center h-14">
                            <BarcodeSample :type="option.value" />
                        </div>
                        <p class="text-xs font-semibold text-zinc-200">{{ option.label }}</p>
                        <p class="text-[11px] text-zinc-500 mt-0.5">{{ option.hint }}</p>
                    </div>
                </div>
            </div>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Product</label>
                    <AdminSelect v-model="productId" :options="productOptions"
                        :placeholder="productsLoading ? 'Loading products...' : 'No linked product'" />
                    <p class="text-xs text-zinc-500 mt-1">Optional — leave unassigned to link it later.</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Barcodes</label>

                    <div v-for="(row, index) in rows" :key="row.key" class="flex items-center gap-2 mb-2">
                        <span class="text-xs text-zinc-500 w-5 shrink-0">{{ index + 1 }}.</span>

                        <div class="flex-1 relative">
                            <input v-model="row.code" type="text" placeholder="Scan or type barcode"
                                :ref="el => setInputRef(row.key, el)" @input="onInput(row)"
                                @blur="() => checkDuplicate(row)" @keydown.enter.prevent="() => onEnter(row, index)"
                                :class="[
                                    'w-full bg-dark-200 border rounded-md px-3 py-2 text-sm text-white placeholder-zinc-600 focus:outline-none',
                                    row.status === 'ok' ? 'border-lime-main' :
                                        row.status === 'error' ? 'border-red-500' : 'border-dark-300'
                                ]" />
                            <Icon v-if="row.status === 'ok'" name="mdi:check-circle"
                                class="absolute right-2 top-1/2 -translate-y-1/2 text-lime-main" size="16" />
                            <Icon v-else-if="row.status === 'error'" name="mdi:alert-circle"
                                class="absolute right-2 top-1/2 -translate-y-1/2 text-red-500" size="16" />
                        </div>

                        <select v-model="row.type" @change="row.typeManual = true"
                            class="bg-dark-200 border border-dark-300 rounded-md px-2 py-2 text-xs text-zinc-300 focus:outline-none shrink-0">
                            <option v-for="option in typeOptions" :key="option.value" :value="option.value">
                                {{ option.label }}
                            </option>
                        </select>

                        <button type="button" @click="() => scanRow(row)" title="Scan with camera"
                            class="p-2 rounded-md text-zinc-400 hover:text-lime-main hover:bg-dark-300 transition-colors shrink-0">
                            <Icon name="mdi:barcode-scan" size="18" />
                        </button>

                        <button type="button" @click="removeRow(index)" :disabled="rows.length === 1"
                            class="p-2 rounded-md text-zinc-400 hover:text-red-400 hover:bg-dark-300 transition-colors disabled:opacity-30 disabled:cursor-not-allowed shrink-0">
                            <Icon name="mdi:trash-can-outline" size="18" />
                        </button>
                    </div>

                    <p v-if="rows.some(r => r.status === 'error')" class="text-xs text-red-400 mt-1">
                        Duplicate codes must be fixed before submitting.
                    </p>

                    <button type="button" @click="addRow"
                        class="mt-1 px-3 py-1.5 rounded-md text-xs font-semibold text-lime-main hover:bg-dark-300 transition-colors">
                        + Add another row
                    </button>
                </div>

                <button type="submit" :disabled="submitting"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit disabled:opacity-50 disabled:cursor-not-allowed">
                    {{ submitting ? 'Adding...' : 'Add barcodes' }}
                </button>
            </form>

            <div v-if="result" class="mt-6 border border-dark-300 rounded-lg bg-dark-100 p-4">
                <p class="text-sm text-white">{{ result.created.length }} barcode(s) added.</p>
                <p v-if="result.skipped.length" class="text-xs text-red-400 mt-1">
                    Skipped (duplicate/already exists): {{ result.skipped.join(', ') }}
                </p>
            </div>
        </div>

        <div v-if="scanModal.open" class="fixed inset-0 z-50 bg-black/70 flex items-center justify-center p-4"
            @click.self="closeScanModal">
            <div class="bg-dark-100 border border-dark-300 rounded-lg p-4 w-full max-w-sm">
                <div class="flex items-center justify-between mb-3">
                    <h3 class="text-sm font-semibold text-white">Scan barcode</h3>
                    <button type="button" @click="closeScanModal" class="text-zinc-400 hover:text-white">
                        <Icon name="mdi:close" size="18" />
                    </button>
                </div>

                <video ref="videoEl" class="w-full rounded-md bg-black aspect-video" autoplay playsinline muted />

                <p class="text-xs text-zinc-500 mt-3">{{ scanModal.supported ? 'Point the camera ' : "No supported" }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const products = ref([]);
const productsLoading = ref(true);
const productId = ref(null);
const legendOpen = ref(true);

const typeOptions = [
    { label: 'Code 128', value: 'code128', hint: 'Variable-length, for internal SKUs' },
    { label: 'EAN-13', value: 'ean13', hint: '13 digits, retail standard outside US' },
    { label: 'UPC-A', value: 'upc_a', hint: '12 digits, US/Canada retail standard' },
    { label: 'QR', value: 'qr', hint: 'Square grid, scanned by camera' }
];

const detectorFormatMap = {
    code_128: 'code128',
    ean_13: 'ean13',
    upc_a: 'upc_a',
    qr_code: 'qr'
};

function detectTypeFromCode(code) {
    if (/^\d{13}$/.test(code)) return 'ean13';
    if (/^\d{12}$/.test(code)) return 'upc_a';
    return 'code128';
}

function makeRow() {
    return { key: crypto.randomUUID(), code: '', type: 'code128', typeManual: false, status: 'idle' };
}

const rows = ref([makeRow()]);
const submitting = ref(false);
const result = ref(null);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const productOptions = computed(() =>
    products.value.map((product) => ({
        label: `${product.name} (${product.sku})`,
        value: product.id
    }))
);

async function fetchProducts() {
    productsLoading.value = true;
    try {
        const data = await authFetch('/api/admin/products/list');
        if (data) {
            products.value = data;
        }
    } catch (e) {
        products.value = [];
    } finally {
        productsLoading.value = false;
    }
}

function addRow() {
    rows.value.push(makeRow());
}

function removeRow(index) {
    if (rows.value.length === 1) return;
    rows.value.splice(index, 1);
}

function onInput(row) {
    row.status = 'idle';
}

function checkDuplicate(row) {
    const code = row.code.trim();
    if (!code) {
        row.status = 'idle';
        return;
    }
    if (!row.typeManual) {
        row.type = detectTypeFromCode(code);
    }
    const duplicate = rows.value.some((r) => r !== row && r.code.trim() === code);
    row.status = duplicate ? 'error' : 'ok';
}

function onEnter(row, index) {
    checkDuplicate(row);
    if (row.status === 'ok' && index === rows.value.length - 1) {
        addRow();
        focusRow(rows.value[rows.value.length - 1].key);
    }
}

const inputRefs = new Map();
function setInputRef(key, el) {
    if (el) inputRefs.set(key, el);
}
function focusRow(key) {
    nextTick(() => {
        inputRefs.get(key)?.focus();
    });
}

const scanModal = ref({ open: false, supported: false, row: null });
const videoEl = ref(null);
let mediaStream = null;
let detectLoop = null;

async function scanRow(row) {
    scanModal.value = { open: true, supported: 'BarcodeDetector' in window, row };

    if (!('BarcodeDetector' in window)) {
        focusRow(row.key);
        return;
    }

    try {
        mediaStream = await navigator.mediaDevices.getUserMedia({
            video: { facingMode: 'environment' }
        });
        await nextTick();
        if (videoEl.value) {
            videoEl.value.srcObject = mediaStream;
        }

        const detector = new window.BarcodeDetector({
            formats: ['code_128', 'ean_13', 'upc_a', 'qr_code']
        });

        const detect = async () => {
            if (!scanModal.value.open || !videoEl.value) return;
            try {
                const barcodes = await detector.detect(videoEl.value);
                if (barcodes.length > 0) {
                    row.code = barcodes[0].rawValue;
                    row.type = detectorFormatMap[barcodes[0].format] || detectTypeFromCode(barcodes[0].rawValue);
                    row.typeManual = false;
                    closeScanModal();
                    checkDuplicate(row);
                    return;
                }
            } catch (e) {
                // keep trying
            }
            detectLoop = requestAnimationFrame(detect);
        };
        detectLoop = requestAnimationFrame(detect);
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = 'Could not access the camera. You can still type or scan with a USB gun.';
        showStatus.value = true;
        setTimeout(() => { showStatus.value = false; }, 4000);
        closeScanModal();
    }
}

function closeScanModal() {
    scanModal.value.open = false;
    if (detectLoop) {
        cancelAnimationFrame(detectLoop);
        detectLoop = null;
    }
    if (mediaStream) {
        mediaStream.getTracks().forEach((t) => t.stop());
        mediaStream = null;
    }
}

onBeforeUnmount(() => {
    closeScanModal();
});

function validate() {
    rows.value.forEach(checkDuplicate);
    return !rows.value.some((r) => r.status === 'error');
}

async function handleSubmit() {
    if (!validate()) return;

    const entries = rows.value
        .map((r) => ({ code: r.code.trim(), barcode_type: r.type }))
        .filter((e) => e.code !== '');

    if (entries.length === 0) return;

    submitting.value = true;
    statusType.value = 'loading';
    statusMessage.value = 'Adding barcodes...';
    showStatus.value = true;
    result.value = null;

    try {
        const data = await authFetch('/api/admin/barcodes/bulk', {
            method: 'POST',
            body: {
                product_id: productId.value,
                barcodes: entries
            }
        });

        result.value = data;
        statusType.value = 'success';
        statusMessage.value = 'Barcodes added.';
        rows.value = [makeRow()];
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to add barcodes.';
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    } finally {
        submitting.value = false;
    }
}

await fetchProducts();
</script>