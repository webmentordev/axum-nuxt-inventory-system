<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center gap-3 mb-6">
            <NuxtLink to="/admin/orders"
                class="p-2 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                <Icon name="mdi:arrow-left" size="20" />
            </NuxtLink>
            <div>
                <h1 class="text-xl font-bold text-white">Order items</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ items.length }} item{{ items.length === 1 ? '' : 's' }}</p>
            </div>
            <div class="ml-auto flex items-center">
                <AdminButton @click="fetchItems()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink :to='`/admin/orders/${route.params.id}/items/create`'
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors ml-2">
                    Add new items
                </NuxtLink>
            </div>
        </div>

        <div v-if="order" class="border border-dark-300 rounded-lg bg-dark-100 p-5 mb-6">
            <div class="flex items-start justify-between">
                <div>
                    <p class="text-xs text-zinc-500">Order number</p>
                    <p class="text-white font-semibold mt-0.5">{{ order.order_number }}</p>
                </div>
                <span class="px-2 py-1 rounded text-xs font-semibold" :class="statusClass(order.status)">
                    {{ statusLabel(order.status) }}
                </span>
            </div>

            <div class="grid grid-cols-4 gap-4 mt-4">
                <div>
                    <p class="text-xs text-zinc-500">CustomerID</p>
                    <p class="text-zinc-200 mt-0.5">{{ order.userId || '-' }}</p>
                </div>
                <div>
                    <p class="text-xs text-zinc-500">Customer</p>
                    <p class="text-zinc-200 mt-0.5">{{ order.customer_name }}</p>
                </div>
                <div>
                    <p class="text-xs text-zinc-500">Email</p>
                    <p class="text-zinc-200 mt-0.5">{{ order.customer_email || '—' }}</p>
                </div>
                <div>
                    <p class="text-xs text-zinc-500">Phone</p>
                    <p class="text-zinc-200 mt-0.5">{{ order.customer_phone || '—' }}</p>
                </div>
                <div class="col-span-4">
                    <p class="text-xs text-zinc-500">Shipping address</p>
                    <p class="text-zinc-200 mt-0.5">{{ order.shipping_address || '—' }}</p>
                </div>
                <div class="col-span-4">
                    <p class="text-xs text-zinc-500">Note</p>
                    <p class="text-zinc-200 mt-0.5">{{ order.notes || '—' }}</p>
                </div>
            </div>
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="items.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Product</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">SKU</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Unit price</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Quantity</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Line total</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Added</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="item in items" :key="item.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ item.product_name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ item.product_sku }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ formatCurrency(item.unit_price) }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ item.quantity }}</td>
                        <td class="px-4 py-3 text-zinc-200">{{ formatCurrency(item.line_total) }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(item.created_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(item.id, el)">
                            <button type="button" @click="toggleMenu(item.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === item.id"
                                class="absolute right-4 top-full mt-1 w-44 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(item)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleDelete(item)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">No items</p>
                <p class="text-zinc-500 text-sm mt-1">Items added to this order will show up here.</p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete item"
            :message="`Are you sure you want to delete ${itemToDelete?.product_name}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();
const route = useRoute();

const items = ref([]);
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const itemToDelete = ref(null);

const order = computed(() => items.value.length ? items.value[0].order : null);

const statusStyles = {
    pending: 'bg-dark-300 text-zinc-300',
    confirmed: 'bg-lime-bg text-lime-main',
    processing: 'bg-lime-bg text-lime-main',
    shipped: 'bg-lime-bg text-lime-main',
    delivered: 'bg-lime-bg text-lime-main',
    cancelled: 'bg-red-950 text-red-400',
    walkin: 'bg-dark-300 text-zinc-300',
    walkin_completed: 'bg-dark-300 text-lime-main',
};

function statusClass(status) {
    return statusStyles[status] || 'bg-dark-300 text-zinc-400';
}

const statusLabels = {
    pending: 'Pending',
    confirmed: 'Confirmed',
    processing: 'Processing',
    shipped: 'Shipped',
    delivered: 'Delivered',
    cancelled: 'Cancelled',
    walkin: 'Walk-in',
    walkin_completed: 'Walk-in Completed'
};

function statusLabel(status) {
    return statusLabels[status] || status;
}

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

function toggleMenu(id) {
    openMenuId.value = openMenuId.value === id ? null : id;
}

function closeMenu() {
    openMenuId.value = null;
}

function handleEdit(item) {
    closeMenu();
    navigateTo(`/admin/orders/${route.params.id}/items/${item.id}/edit`);
}

function handleDelete(item) {
    closeMenu();
    itemToDelete.value = item;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const item = itemToDelete.value;
    if (!item) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting item...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/orders/${route.params.id}/items/${item.id}`, {
            method: 'DELETE'
        });
        items.value = items.value.filter((i) => i.id !== item.id);
        statusType.value = 'success';
        statusMessage.value = 'Item deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete item.';
    } finally {
        itemToDelete.value = null;
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

async function fetchItems() {
    try {
        const data = await authFetch(`/api/admin/orders/${route.params.id}/items`);
        if (data) {
            items.value = data;
        }
    } catch (e) {
        items.value = [];
    }
}

function formatCurrency(amount) {
    return new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: 'PKR'
    }).format(Number(amount));
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

await fetchItems();
</script>