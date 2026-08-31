<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Orders</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ orders.length }} total</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchOrders()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink to="/admin/orders/create"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors ml-2">
                    Add order
                </NuxtLink>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by order number, customer or email..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredOrders.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Order #</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Items</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Quantity</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Customer</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Contact</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Total</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Updated</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="order in filteredOrders" :key="order.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ order.order_number }}</td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ order.total_items }}</td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ order.total_quantity }}</td>
                        <td class="px-4 py-3 text-zinc-200">{{ order.customer_name }}</td>
                        <td class="px-4 py-3 text-zinc-400">
                            <div>{{ order.customer_email || '—' }}</div>
                            <div class="text-xs text-zinc-500">{{ order.customer_phone || '' }}</div>
                        </td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="statusClass(order.status)">
                                {{ statusLabel(order.status) }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-200">{{ formatCurrency(order.total_amount) }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(order.created_at) }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ order.created_at ==
                            order.updated_at ? '-' : formatDate(order.updated_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(order.id, el)">
                            <button type="button" @click="toggleMenu(order.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === order.id"
                                class="absolute right-4 top-full mt-1 w-44 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(order)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleViewItems(order)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    View order
                                </button>
                                <button type="button" @click="handleAddItems(order)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Add items
                                </button>
                                <button type="button" @click="handleDelete(order)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching orders' : 'No orders' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Orders you create will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete order"
            :message="`Are you sure you want to delete order ${orderToDelete?.order_number}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const orders = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const orderToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

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

const filteredOrders = computed(() => {
    if (!search.value.trim()) return orders.value;
    const query = search.value.trim().toLowerCase();
    return orders.value.filter((order) =>
        order.id.toLowerCase().includes(query) ||
        order.user_id?.toLowerCase().includes(query) ||
        order.order_number.toLowerCase().includes(query) ||
        order.customer_name.toLowerCase().includes(query) ||
        order.shipping_address?.toLowerCase().includes(query) ||
        order.status.toLowerCase().includes(query) ||
        order.notes?.toLowerCase().includes(query) ||
        (order.customer_email || '').toLowerCase().includes(query) ||
        (order.customer_phone || '').toLowerCase().includes(query)
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

async function fetchOrders() {
    try {
        const data = await authFetch('/api/admin/orders');
        if (data) {
            orders.value = data;
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

function toggleMenu(id) {
    openMenuId.value = openMenuId.value === id ? null : id;
}

function closeMenu() {
    openMenuId.value = null;
}

function handleViewItems(order) {
    closeMenu();
    navigateTo(`/admin/orders/${order.id}/items`);
}

function handleAddItems(order) {
    closeMenu();
    navigateTo(`/admin/orders/${order.id}/items/create`);
}

function handleEdit(order) {
    closeMenu();
    navigateTo(`/admin/orders/${order.id}/edit`);
}

function handleDelete(order) {
    closeMenu();
    orderToDelete.value = order;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const order = orderToDelete.value;
    if (!order) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting order...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/orders/${order.id}`, {
            method: 'DELETE'
        });
        orders.value = orders.value.filter((o) => o.id !== order.id);
        statusType.value = 'success';
        statusMessage.value = 'Order deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete order.';
    } finally {
        orderToDelete.value = null;
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

function formatCurrency(amount) {
    const currency = useRuntimeConfig().public.currency;
    return new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: currency
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

await fetchOrders();
</script>