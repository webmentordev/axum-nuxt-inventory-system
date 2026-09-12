<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Barcodes</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ barcodes.length }} total</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchAll()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink to="/admin/barcodes/add"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors ml-2">
                    Add barcodes
                </NuxtLink>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by code, product or type..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredBarcodes.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Code</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Type</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Product</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="barcode in filteredBarcodes" :key="barcode.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-200 font-mono">{{ barcode.code }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ typeLabel(barcode.barcode_type) }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ productName(barcode.product_id) }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="barcode.is_sold
                                ? 'bg-dark-300 text-zinc-400'
                                : 'bg-lime-bg text-lime-main'">
                                {{ barcode.is_sold ? 'Sold' : 'Available' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(barcode.created_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(barcode.id, el)">
                            <button type="button" @click="toggleMenu(barcode.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === barcode.id"
                                class="absolute right-4 top-full mt-1 w-56 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">

                                <template v-if="assigningId !== barcode.id">
                                    <button type="button" @click="startAssign(barcode)"
                                        class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                        {{ barcode.product_id ? 'Reassign product' : 'Assign product' }}
                                    </button>
                                    <button v-if="barcode.product_id" type="button" @click="handleUnassign(barcode)"
                                        class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                        Unassign product
                                    </button>
                                    <button type="button" @click="handleToggleSold(barcode)"
                                        class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                        {{ barcode.is_sold ? 'Mark available' : 'Mark sold' }}
                                    </button>
                                    <button type="button" @click="handleDelete(barcode)"
                                        class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                        Delete
                                    </button>
                                </template>

                                <template v-else>
                                    <div class="p-3">
                                        <AdminSelect v-model="assignProductId" :options="productOptions"
                                            placeholder="Select product" />
                                        <div class="flex items-center gap-2 mt-2">
                                            <button type="button" @click="confirmAssign(barcode)"
                                                class="flex-1 px-3 py-1.5 rounded-md text-xs font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors">
                                                Save
                                            </button>
                                            <button type="button" @click="cancelAssign"
                                                class="flex-1 px-3 py-1.5 rounded-md text-xs font-semibold text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                                Cancel
                                            </button>
                                        </div>
                                    </div>
                                </template>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching barcodes' : 'No barcodes' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Barcodes you add will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete barcode"
            :message="`Are you sure you want to delete ${barcodeToDelete?.code}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const barcodes = ref([]);
const products = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const assigningId = ref(null);
const assignProductId = ref(null);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const barcodeToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const typeLabels = {
    code128: 'Code 128',
    ean13: 'EAN-13',
    upc_a: 'UPC-A',
    qr: 'QR'
};

function typeLabel(type) {
    return typeLabels[type] || type;
}

const productMap = computed(() => {
    const map = {};
    for (const p of products.value) {
        map[p.id] = p;
    }
    return map;
});

function productName(productId) {
    if (!productId) return 'Unassigned';
    return productMap.value[productId]?.name || 'Unknown product';
}

const productOptions = computed(() =>
    products.value.map((product) => ({
        label: product.name,
        value: product.id
    }))
);

const filteredBarcodes = computed(() => {
    if (!search.value.trim()) return barcodes.value;
    const query = search.value.trim().toLowerCase();
    return barcodes.value.filter((barcode) =>
        barcode.code.toLowerCase().includes(query) ||
        typeLabel(barcode.barcode_type).toLowerCase().includes(query) ||
        productName(barcode.product_id).toLowerCase().includes(query)
    );
});

function setMenuRef(id, el) {
    if (el) {
        menuRefs.value[id] = el;
    } else {
        delete menuRefs.value[id];
    }
}

const activeMenuEl = computed(() => menuRefs.value[openMenuId.value] || null);

onClickOutside(activeMenuEl, () => {
    closeMenu();
});

async function fetchBarcodes() {
    try {
        const data = await authFetch('/api/admin/barcodes');
        if (data) {
            barcodes.value = data;
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

async function fetchProducts() {
    try {
        const data = await authFetch('/api/admin/products/list');
        if (data) {
            products.value = data;
        }
    } catch (e) {
        products.value = [];
    }
}

async function fetchAll() {
    await Promise.all([fetchBarcodes(), fetchProducts()]);
}

function toggleMenu(id) {
    openMenuId.value = openMenuId.value === id ? null : id;
    assigningId.value = null;
}

function closeMenu() {
    openMenuId.value = null;
    assigningId.value = null;
}

function startAssign(barcode) {
    assigningId.value = barcode.id;
    assignProductId.value = barcode.product_id;
}

function cancelAssign() {
    assigningId.value = null;
}

async function confirmAssign(barcode) {
    if (!assignProductId.value) return;
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = 'Assigning product...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/barcodes/${barcode.id}`, {
            method: 'PATCH',
            body: { product_id: assignProductId.value }
        });
        barcode.product_id = assignProductId.value;
        statusType.value = 'success';
        statusMessage.value = 'Product assigned.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to assign product.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

async function handleUnassign(barcode) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = 'Unassigning product...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/barcodes/${barcode.id}`, {
            method: 'PATCH',
            body: { product_id: null }
        });
        barcode.product_id = null;
        statusType.value = 'success';
        statusMessage.value = 'Product unassigned.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to unassign product.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

async function handleToggleSold(barcode) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = barcode.is_sold ? 'Marking available...' : 'Marking sold...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/barcodes/${barcode.id}`, {
            method: 'PATCH',
            body: { is_sold: !barcode.is_sold }
        });
        barcode.is_sold = !barcode.is_sold;
        statusType.value = 'success';
        statusMessage.value = barcode.is_sold ? 'Marked sold.' : 'Marked available.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update barcode.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

function handleDelete(barcode) {
    closeMenu();
    barcodeToDelete.value = barcode;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const barcode = barcodeToDelete.value;
    if (!barcode) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting barcode...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/barcodes/${barcode.id}`, {
            method: 'DELETE'
        });
        barcodes.value = barcodes.value.filter((b) => b.id !== barcode.id);
        statusType.value = 'success';
        statusMessage.value = 'Barcode deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete barcode.';
    } finally {
        barcodeToDelete.value = null;
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

function formatDate(utcString) {
    return new Date(utcString).toLocaleString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: 'numeric',
        minute: '2-digit'
    });
}

await fetchAll();
</script>