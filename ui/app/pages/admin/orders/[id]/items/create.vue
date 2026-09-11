<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center gap-3 mb-6">
            <NuxtLink :to="`/admin/orders/${orderUuid}/items`"
                class="p-2 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                <Icon name="mdi:arrow-left" size="20" />
            </NuxtLink>
            <div>
                <h1 class="text-xl font-bold text-white">Add items</h1>
                <p class="text-sm text-zinc-500 mt-1">Add products to this order. Barcodes are optional.</p>
            </div>
        </div>

        <div class="max-w-3xl">
            <div class="border border-dark-300 rounded-lg bg-dark-100 p-5">
                <div v-for="(row, index) in rows" :key="row.key"
                    class="border-b border-dark-300 pb-4 mb-4 last:border-b-0 last:pb-0 last:mb-0">

                    <div class="grid grid-cols-[1fr_140px_40px] gap-3 items-start">
                        <div>
                            <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Product</label>
                            <AdminSelect v-model="row.product_id" :options="productOptions"
                                :placeholder="productsLoading ? 'Loading products...' : 'Select a product'"
                                @update:modelValue="() => onProductChange(row)" />
                            <p v-if="row.error" class="text-xs text-red-400 mt-1">{{ row.error }}</p>
                        </div>

                        <div>
                            <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Quantity</label>
                            <AdminInput v-model="row.quantity" type="number" min="1" placeholder="1"
                                @update:modelValue="() => onQuantityChange(row)" />
                        </div>

                        <div class="pt-6">
                            <button type="button" @click="removeRow(index)" :disabled="rows.length === 1"
                                class="p-2 rounded-md text-zinc-400 hover:text-red-400 hover:bg-dark-300 transition-colors disabled:opacity-30 disabled:cursor-not-allowed">
                                <Icon name="mdi:trash-can-outline" size="18" />
                            </button>
                        </div>
                    </div>

                    <div v-if="row.product_id && row.barcodes.length" class="mt-3 pl-1">
                        <label class="block text-xs font-semibold text-zinc-400 mb-1.5">
                            Barcodes <span class="font-normal text-zinc-600">(optional — leave blank if this unit has no
                                barcode)</span>
                        </label>

                        <div v-for="(bc, bIndex) in row.barcodes" :key="bc.key" class="flex items-center gap-2 mb-2">
                            <span class="text-xs text-zinc-500 w-5 shrink-0">{{ bIndex + 1 }}.</span>

                            <div class="flex-1 relative">
                                <input v-model="bc.value" type="text" placeholder="Scan or type barcode"
                                    ref="el => setBarcodeInputRef(row.key, bIndex, el)" @input="onBarcodeInput(row, bc)"
                                    @blur="() => verifyBarcode(row, bc)"
                                    @keydown.enter.prevent="() => verifyBarcode(row, bc)" :class="[
                                        'w-full bg-dark-200 border rounded-md px-3 py-2 text-sm text-white placeholder-zinc-600 focus:outline-none',
                                        bc.status === 'valid' ? 'border-lime-main' :
                                            bc.status === 'invalid' ? 'border-red-500' : 'border-dark-300'
                                    ]" />
                                <Icon v-if="bc.status === 'checking'" name="mdi:loading"
                                    class="absolute right-2 top-1/2 -translate-y-1/2 animate-spin text-zinc-500"
                                    size="16" />
                                <Icon v-else-if="bc.status === 'valid'" name="mdi:check-circle"
                                    class="absolute right-2 top-1/2 -translate-y-1/2 text-lime-main" size="16" />
                                <Icon v-else-if="bc.status === 'invalid'" name="mdi:alert-circle"
                                    class="absolute right-2 top-1/2 -translate-y-1/2 text-red-500" size="16" />
                            </div>

                            <button type="button" @click="() => scanBarcode(row, bc)" title="Scan with camera"
                                class="p-2 rounded-md text-zinc-400 hover:text-lime-main hover:bg-dark-300 transition-colors shrink-0">
                                <Icon name="mdi:barcode-scan" size="18" />
                            </button>

                            <button v-if="bc.value" type="button" @click="() => clearBarcode(row, bc)" title="Clear"
                                class="p-2 rounded-md text-zinc-400 hover:text-red-400 hover:bg-dark-300 transition-colors shrink-0">
                                <Icon name="mdi:close" size="16" />
                            </button>
                        </div>

                        <p v-if="row.barcodes.some(b => b.status === 'invalid')" class="text-xs text-red-400 mt-1">
                            Fix or clear the invalid barcode(s) above before submitting.
                        </p>
                    </div>
                </div>

                <button type="button" @click="addRow"
                    class="mt-2 px-3 py-1.5 rounded-md text-xs font-semibold text-lime-main hover:bg-dark-300 transition-colors">
                    + Add another item
                </button>
            </div>

            <div class="flex items-center gap-3 mt-6">
                <button type="button" @click="handleSubmit" :disabled="submitting"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                    {{ submitting ? 'Adding...' : 'Add items' }}
                </button>
                <NuxtLink :to="`/admin/orders/${orderUuid}/items`"
                    class="px-4 py-2 rounded-md text-sm font-semibold text-zinc-400 hover:text-white transition-colors">
                    Cancel
                </NuxtLink>
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
                <p class="text-xs text-zinc-500 mt-3">{{ scanModal.supported ? 'Point the camera at the barcode.' :
                    "Your browser can't scan with the camera" }}</p>
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
const route = useRoute();

const orderUuid = route.params.id;

const productList = ref([]);
const productsLoading = ref(true);

function makeBarcodeField() {
    return { key: crypto.randomUUID(), value: '', status: 'idle', message: '' };
}

function makeRow() {
    return { key: crypto.randomUUID(), product_id: null, quantity: 1, error: '', barcodes: [] };
}

const rows = ref([makeRow()]);

const submitting = ref(false);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const activeProducts = computed(() => productList.value.filter((p) => p.is_active));

const productOptions = computed(() =>
    activeProducts.value.map((product) => ({
        label: `${product.name} — ${product.quantity_in_stock} in stock`,
        value: product.id
    }))
);

async function fetchProducts() {
    productsLoading.value = true;
    try {
        const data = await authFetch('/api/admin/products/list');
        if (data) {
            productList.value = data;
        }
    } catch (e) {
        productList.value = [];
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

function syncBarcodeCount(row) {
    const qty = Math.max(0, Number(row.quantity) || 0);
    if (row.barcodes.length < qty) {
        while (row.barcodes.length < qty) {
            row.barcodes.push(makeBarcodeField());
        }
    } else if (row.barcodes.length > qty) {
        row.barcodes.splice(qty);
    }
}

function onQuantityChange(row) {
    syncBarcodeCount(row);
}

function onProductChange(row) {
    row.barcodes = [];
    syncBarcodeCount(row);
}

function onBarcodeInput(row, bc) {
    if (bc.status !== 'idle') {
        bc.status = 'idle';
        bc.message = '';
    }
}

function clearBarcode(row, bc) {
    bc.value = '';
    bc.status = 'idle';
    bc.message = '';
}

let verifyTimer = null;
async function verifyBarcode(row, bc) {
    const code = bc.value.trim();
    if (!code) {
        bc.status = 'idle';
        bc.message = '';
        return;
    }
    if (!row.product_id) {
        bc.status = 'invalid';
        bc.message = 'Select a product first.';
        return;
    }

    const duplicate = rows.value.some((r) =>
        r.barcodes.some((other) => other !== bc && other.value.trim() === code)
    );
    if (duplicate) {
        bc.status = 'invalid';
        bc.message = 'This barcode is already used elsewhere in this order.';
        return;
    }

    bc.status = 'checking';
    clearTimeout(verifyTimer);

    try {
        const params = new URLSearchParams({ code, product_id: row.product_id });
        const result = await authFetch(`/api/admin/orders/barcodes/lookup?${params.toString()}`);
        if (result?.valid) {
            bc.status = 'valid';
            bc.message = '';
        } else {
            bc.status = 'invalid';
            bc.message = result?.reason || 'Invalid barcode.';
        }
    } catch (e) {
        bc.status = 'invalid';
        bc.message = 'Could not verify barcode.';
    }
}

const scanModal = ref({ open: false, supported: false, row: null, field: null });
const videoEl = ref(null);
let mediaStream = null;
let detectLoop = null;

async function scanBarcode(row, bc) {
    scanModal.value = {
        open: true,
        supported: 'BarcodeDetector' in window,
        row,
        field: bc
    };

    if (!('BarcodeDetector' in window)) {
        closeScanModal();
        focusBarcodeField(row.key, row.barcodes.indexOf(bc));
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
                    bc.value = barcodes[0].rawValue;
                    closeScanModal();
                    await verifyBarcode(row, bc);
                    return;
                }
            } catch (e) {
            }
            detectLoop = requestAnimationFrame(detect);
        };
        detectLoop = requestAnimationFrame(detect);
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = 'Could not access the camera. You can still type the barcode manually.';
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

const barcodeInputRefs = new Map();
function setBarcodeInputRef(rowKey, index, el) {
    if (el) barcodeInputRefs.set(`${rowKey}:${index}`, el);
}
function focusBarcodeField(rowKey, index) {
    nextTick(() => {
        barcodeInputRefs.get(`${rowKey}:${index}`)?.focus();
    });
}

onBeforeUnmount(() => {
    closeScanModal();
});

function validate() {
    let valid = true;
    for (const row of rows.value) {
        row.error = '';
        const product = productList.value.find((p) => p.id === row.product_id);

        if (!row.product_id) {
            row.error = 'Select a product.';
            valid = false;
        } else if (!row.quantity || Number(row.quantity) <= 0) {
            row.error = 'Quantity must be at least 1.';
            valid = false;
        } else if (product && Number(row.quantity) > product.quantity_in_stock) {
            row.error = `Only ${product.quantity_in_stock} in stock.`;
            valid = false;
        }

        if (row.barcodes.some((b) => b.status === 'invalid')) {
            valid = false;
        }
    }
    return valid;
}

async function handleSubmit() {
    if (!validate()) return;

    submitting.value = true;
    statusType.value = 'loading';
    statusMessage.value = 'Adding items...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/orders/${orderUuid}/items`, {
            method: 'POST',
            body: {
                items: rows.value.map((row) => ({
                    product_id: row.product_id,
                    quantity: Number(row.quantity),
                    barcodes: row.barcodes
                        .map((b) => b.value.trim())
                        .filter((v) => v !== '')
                }))
            }
        });

        statusType.value = 'success';
        statusMessage.value = 'Items added.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to add items.';
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    } finally {
        submitting.value = false;
    }
}

await fetchProducts();
</script>