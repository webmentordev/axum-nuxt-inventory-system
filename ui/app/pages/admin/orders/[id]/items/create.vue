<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center gap-3 mb-6">
            <NuxtLink :to="`/admin/orders/${orderUuid}/items`"
                class="p-2 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                <Icon name="mdi:arrow-left" size="20" />
            </NuxtLink>
            <div>
                <h1 class="text-xl font-bold text-white">Add items</h1>
                <p class="text-sm text-zinc-500 mt-1">Add products to this order.</p>
            </div>
        </div>

        <div class="max-w-3xl">
            <div class="border border-dark-300 rounded-lg bg-dark-100 p-5">
                <div v-for="(row, index) in rows" :key="row.key"
                    class="grid grid-cols-[1fr_140px_40px] gap-3 items-start mb-4 last:mb-0">
                    <div>
                        <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Product</label>
                        <select v-model="row.product_id"
                            class="w-full rounded-md bg-dark-200 border border-dark-300 px-3 py-2 text-sm text-zinc-200 focus:outline-none focus:border-lime-main">
                            <option value="" disabled>Select a product</option>
                            <option v-for="product in activeProducts" :key="product.id" :value="product.id">
                                {{ product.name }} — {{ product.quantity_in_stock }} in stock
                            </option>
                        </select>
                        <p v-if="row.error" class="text-xs text-red-400 mt-1">{{ row.error }}</p>
                    </div>

                    <div>
                        <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Quantity</label>
                        <AdminInput v-model="row.quantity" type="number" min="1" placeholder="1" />
                    </div>

                    <div class="pt-6">
                        <button type="button" @click="removeRow(index)" :disabled="rows.length === 1"
                            class="p-2 rounded-md text-zinc-400 hover:text-red-400 hover:bg-dark-300 transition-colors disabled:opacity-30 disabled:cursor-not-allowed">
                            <Icon name="mdi:trash-can-outline" size="18" />
                        </button>
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
const rows = ref([{ key: crypto.randomUUID(), product_id: '', quantity: 1, error: '' }]);

const submitting = ref(false);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const activeProducts = computed(() => productList.value.filter((p) => p.is_active));

async function fetchProducts() {
    try {
        const data = await authFetch('/api/admin/products/list');
        if (data) {
            productList.value = data;
        }
    } catch (e) {
        productList.value = [];
    }
}

function addRow() {
    rows.value.push({ key: crypto.randomUUID(), product_id: '', quantity: 1, error: '' });
}

function removeRow(index) {
    if (rows.value.length === 1) return;
    rows.value.splice(index, 1);
}

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
                    quantity: Number(row.quantity)
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