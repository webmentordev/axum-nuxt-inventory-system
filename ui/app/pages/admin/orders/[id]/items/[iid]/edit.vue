<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center gap-3 mb-6">
            <NuxtLink :to="`/admin/orders/${orderUuid}/items`"
                class="p-2 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                <Icon name="mdi:arrow-left" size="20" />
            </NuxtLink>
            <div>
                <h1 class="text-xl font-bold text-white">Edit item</h1>
                <p class="text-sm text-zinc-500 mt-1">Update the product, quantity, or status for this item.</p>
            </div>
        </div>

        <div class="max-w-3xl">
            <div v-if="loading" class="border border-dark-300 rounded-lg bg-dark-100 p-5 text-sm text-zinc-400">
                Loading item...
            </div>

            <template v-else>
                <div class="border border-dark-300 rounded-lg bg-dark-100 p-5">
                    <div class="grid grid-cols-[1fr_140px] gap-3 items-start">
                        <div>
                            <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Product</label>
                            <AdminSelect v-model="row.product_id" :options="productOptions" :disabled="!isSold"
                                :placeholder="productsLoading ? 'Loading products...' : 'Select a product'" />
                            <p v-if="row.error" class="text-xs text-red-400 mt-1">{{ row.error }}</p>
                        </div>

                        <div>
                            <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Quantity</label>
                            <AdminInput v-model="row.quantity" type="number" min="1" placeholder="1"
                                :disabled="!isSold" />
                        </div>
                    </div>

                    <p v-if="!isSold" class="text-xs text-zinc-500 mt-3">
                        Product and quantity can't be changed once an item is marked {{ statusLabel(row.status) }}.
                    </p>
                </div>

                <div class="border border-dark-300 rounded-lg bg-dark-100 p-5 mt-4">
                    <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Status</label>
                    <div class="flex items-center gap-3">
                        <AdminSelect v-model="row.status" :options="statusOptions" class="max-w-[220px]" />
                        <span :class="statusBadgeClass(row.status)" class="text-xs font-semibold px-2 py-1 rounded-md">
                            {{ statusLabel(row.status) }}
                        </span>
                    </div>
                    <p class="text-xs text-zinc-500 mt-2">
                        Marking as refunded restocks the unit. Refunded (defective) and defective do not restock.
                    </p>
                    <div class="flex justify-end mt-3">
                        <button type="button" @click="handleStatusSubmit"
                            :disabled="statusSubmitting || row.status === originalStatus"
                            class="px-3 py-1.5 rounded-md text-xs font-semibold bg-dark-300 text-white hover:bg-dark-200 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                            {{ statusSubmitting ? 'Updating...' : 'Update status' }}
                        </button>
                    </div>
                </div>

                <div class="flex items-center gap-3 mt-6">
                    <button type="button" @click="handleSubmit" :disabled="submitting || !isSold"
                        class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                        {{ submitting ? 'Saving...' : 'Save changes' }}
                    </button>
                    <NuxtLink :to="`/admin/orders/${orderUuid}/items`"
                        class="px-4 py-2 rounded-md text-sm font-semibold text-zinc-400 hover:text-white transition-colors">
                        Cancel
                    </NuxtLink>
                </div>
            </template>
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
const itemUuid = route.params.iid;

const productList = ref([]);
const productsLoading = ref(true);
const loading = ref(true);
const row = ref({ product_id: null, quantity: 1, status: 'sold', error: '' });
const originalStatus = ref('sold');

const submitting = ref(false);
const statusSubmitting = ref(false);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const statusOptions = [
    { label: 'Sold', value: 'sold' },
    { label: 'Refunded', value: 'refunded' },
    { label: 'Refunded (defective)', value: 'refunded_defective' },
    { label: 'Defective', value: 'defective' }
];

const isSold = computed(() => row.value.status === 'sold');

function statusLabel(value) {
    return statusOptions.find((o) => o.value === value)?.label || value;
}

function statusBadgeClass(value) {
    switch (value) {
        case 'sold':
            return 'bg-lime-main/10 text-lime-main';
        case 'refunded':
            return 'bg-amber-400/10 text-amber-400';
        case 'refunded_defective':
        case 'defective':
            return 'bg-red-400/10 text-red-400';
        default:
            return 'bg-dark-300 text-zinc-400';
    }
}

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

async function fetchItem() {
    loading.value = true;
    try {
        const data = await authFetch(`/api/admin/orders/${orderUuid}/items/${itemUuid}`);
        if (data) {
            row.value.product_id = data.product_id;
            row.value.quantity = data.quantity;
            row.value.status = data.status;
            originalStatus.value = data.status;
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to load item.';
        showStatus.value = true;
    } finally {
        loading.value = false;
    }
}

function validate() {
    row.value.error = '';
    const product = productList.value.find((p) => p.id === row.value.product_id);

    if (!row.value.product_id) {
        row.value.error = 'Select a product.';
        return false;
    }
    if (!row.value.quantity || Number(row.value.quantity) <= 0) {
        row.value.error = 'Quantity must be at least 1.';
        return false;
    }
    if (product && Number(row.value.quantity) > product.quantity_in_stock) {
        row.value.error = `Only ${product.quantity_in_stock} in stock.`;
        return false;
    }
    return true;
}

async function handleSubmit() {
    if (!isSold.value) return;
    if (!validate()) return;

    submitting.value = true;
    statusType.value = 'loading';
    statusMessage.value = 'Saving item...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/orders/${orderUuid}/items/${itemUuid}`, {
            method: 'PATCH',
            body: {
                product_id: row.value.product_id,
                quantity: Number(row.value.quantity)
            }
        });

        statusType.value = 'success';
        statusMessage.value = 'Item updated.';
        setTimeout(() => {
            navigateTo(`/admin/orders/${orderUuid}/items`);
        }, 800);
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update item.';
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    } finally {
        submitting.value = false;
    }
}

async function handleStatusSubmit() {
    if (row.value.status === originalStatus.value) return;

    statusSubmitting.value = true;
    statusType.value = 'loading';
    statusMessage.value = 'Updating status...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/orders/${orderUuid}/items/${itemUuid}/status`, {
            method: 'PATCH',
            body: { status: row.value.status }
        });

        originalStatus.value = row.value.status;
        statusType.value = 'success';
        statusMessage.value = 'Status updated.';
        setTimeout(() => {
            showStatus.value = false;
        }, 1500);
    } catch (e) {
        row.value.status = originalStatus.value;
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update status.';
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    } finally {
        statusSubmitting.value = false;
    }
}

await Promise.all([fetchProducts(), fetchItem()]);
</script>