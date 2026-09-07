<template>
    <section class="w-full min-h-[80vh]">
        <div class="max-w-3xl m-auto p-3 py-12">
            <h1 class="text-2xl font-semibold mb-6">My Orders</h1>

            <Loading v-if="processing" message="Fetching your orders..." />
            <AlertsError v-if="errors.message" :message="errors.message" />

            <div v-if="!processing && !errors.message && orders.length === 0" class="text-center py-12">
                <p class="text-para-light">You haven't placed any orders yet.</p>
                <NuxtLink to="/" class="text-navy underline mt-2 inline-block">Start shopping</NuxtLink>
            </div>

            <div v-if="!processing && orders.length" class="flex flex-col gap-4">
                <div v-for="order in orders" :key="order.id" class="border border-gray-200 rounded-xl overflow-hidden">
                    <button @click="toggleOrder(order.id)"
                        class="w-full flex items-center justify-between p-4 hover:bg-gray-50">
                        <div class="text-left">
                            <p class="text-lg font-semibold">#{{ order.order_number }}</p>
                            <p class="text-sm text-para-light">{{ formatDate(order.created_at) }}</p>
                        </div>
                        <div class="flex items-center gap-3">
                            <span :class="statusClass(order.status)"
                                class="px-3 py-1 rounded-full text-xs font-medium capitalize">
                                {{ order.status == 'walkin' ? 'In Person-Order' : order.status }}
                            </span>
                            <img :class="expanded.has(order.id) ? 'rotate-180' : ''" class="transition-transform"
                                src="https://api.iconify.design/line-md:chevron-down.svg?color=%23374151" width="18">
                        </div>
                    </button>

                    <div v-if="expanded.has(order.id)" class="p-4 border-t border-gray-200 flex flex-col gap-4">
                        <div>
                            <p class="font-medium mb-2">Items</p>
                            <div v-for="item in order.items" :key="item.id" class="mb-3">
                                <div class="flex justify-between text-sm py-1">
                                    <span>{{ item.product_name }} × {{ item.quantity }}</span>
                                    <span>{{ formatCurrency(item.line_total) }}</span>
                                </div>
                                <span v-if="item.status != 'sold'" class="px-2 py-1 rounded text-xs font-semibold"
                                    :class="itemStatusClass(item.status)">
                                    {{ itemStatusLabel(item.status) }}
                                </span>
                            </div>

                        </div>

                        <div class="border-t border-gray-200 pt-4 flex flex-col gap-1 text-sm">
                            <div class="flex justify-between">
                                <span>Subtotal</span>
                                <span>{{ formatCurrency(order.subtotal) }}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>Tax</span>
                                <span>{{ formatCurrency(order.tax_amount) }}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>Shipping</span>
                                <span>{{ formatCurrency(order.shipping_amount) }}</span>
                            </div>
                            <div class="flex justify-between font-semibold text-base pt-2 border-t border-gray-200">
                                <span>Total</span>
                                <span>{{ formatCurrency(order.total_amount) }}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>

<script setup>
definePageMeta({
    middleware: 'auth',
    layout: 'client'
});

const { authFetch } = useAuthFetch();
const config = useRuntimeConfig().public;

const orders = ref([]);
const processing = ref(false);
const errors = reactive({ message: '' });
const expanded = ref(new Set());

const statusColors = {
    pending: 'bg-yellow-100 text-yellow-800',
    confirmed: 'bg-blue-100 text-blue-800',
    processing: 'bg-indigo-100 text-indigo-800',
    shipped: 'bg-purple-100 text-purple-800',
    delivered: 'bg-green-100 text-green-800',
    cancelled: 'bg-red-100 text-red-800',
    walkin: 'bg-gray-100 text-gray-800',
};

const itemStatusStyles = {
    refunded: 'bg-amber-950 text-amber-400',
    refunded_defective: 'bg-red-950 text-red-400',
    defective: 'bg-red-950 text-red-400'
};

function itemStatusClass(status) {
    return itemStatusStyles[status] || 'bg-dark-300 text-zinc-400';
}

const itemStatusLabels = {
    refunded: 'Refunded',
    refunded_defective: 'Refunded (defective)',
    defective: 'Defective'
};

function itemStatusLabel(status) {
    return itemStatusLabels[status] || status;
}

const statusClass = (status) => statusColors[status] || 'bg-gray-100 text-gray-800';

const formatCurrency = (value) => {
    const n = Number(value);
    if (Number.isNaN(n)) return value;
    return `${config.currency} ${n.toLocaleString('en-PK', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
};

const formatDate = (value) => new Date(value).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric'
});

const toggleOrder = (id) => {
    if (expanded.value.has(id)) {
        expanded.value.delete(id);
    } else {
        expanded.value.add(id);
    }
    expanded.value = new Set(expanded.value);
};

const fetchOrders = async () => {
    processing.value = true;
    errors.message = '';
    try {
        const data = await authFetch('/api/auth/orders'); // Auth is required.
        orders.value = data || [];
    } catch (e) {
        errors.message = e?.data?.message || 'Something went wrong';
    } finally {
        processing.value = false;
    }
};

onMounted(fetchOrders);
</script>