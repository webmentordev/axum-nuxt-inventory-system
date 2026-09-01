<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center gap-3 mb-6">
            <NuxtLink :to="`/admin/orders/${orderUuid}/items`"
                class="p-2 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                <Icon name="mdi:arrow-left" size="20" />
            </NuxtLink>
            <div>
                <h1 class="text-xl font-bold text-white">Edit item</h1>
                <p class="text-sm text-zinc-500 mt-1">Update the product or quantity for this item.</p>
            </div>
        </div>

        <div class="max-w-3xl">
            <div v-if="loading" class="border border-dark-300 rounded-lg bg-dark-100 p-5 text-sm text-zinc-400">
                Loading item...
            </div>

            <div v-else class="border border-dark-300 rounded-lg bg-dark-100 p-5">
                <div class="grid grid-cols-[1fr_140px] gap-3 items-start">
                    <div>
                        <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Product</label>
                        <AdminSelect v-model="row.product_id" :options="productOptions"
                            :placeholder="productsLoading ? 'Loading products...' : 'Select a product'" />
                        <p v-if="row.error" class="text-xs text-red-400 mt-1">{{ row.error }}</p>
                    </div>

                    <div>
                        <label class="block text-xs font-semibold text-zinc-400 mb-1.5">Quantity</label>
                        <AdminInput v-model="row.quantity" type="number" min="1" placeholder="1" />
                    </div>
                </div>
            </div>

            <div v-if="!loading" class="flex items-center gap-3 mt-6">
                <button type="button" @click="handleSubmit" :disabled="submitting"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                    {{ submitting ? 'Saving...' : 'Save changes' }}
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
const itemUuid = route.params.iid;

const productList = ref([]);
const productsLoading = ref(true);
const loading = ref(true);
const row = ref({ product_id: null, quantity: 1, error: '' });

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

async function fetchItem() {
    loading.value = true;
    try {
        const data = await authFetch(`/api/admin/orders/${orderUuid}/items/${itemUuid}`);
        if (data) {
            row.value.product_id = data.product_id;
            row.value.quantity = data.quantity;
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

await Promise.all([fetchProducts(), fetchItem()]);
</script>