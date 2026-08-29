<template>
    <div class="w-full min-h-[80vh]">
        <div class="w-full h-full flex items-center justify-center py-12">
            <div class="flex flex-col max-w-87.5 w-full">
                <div class="flex items-center m-auto mb-6">
                    <img src="/kaleem-solar-logo-t-2.png" alt="Kaleem solat logo" width="190px">
                </div>

                <Loading v-if="processing" message="Loading your order..." />

                <AlertsError v-else-if="errors.message" :message="errors.message" />

                <div v-else-if="order" class="flex flex-col gap-4">
                    <div class="flex flex-col items-center text-center gap-2 mb-2">
                        <img class="w-14 h-14"
                            src="https://api.iconify.design/line-md:confirm-circle.svg?color=%2316a34a" width="56">
                        <h1 class="text-xl font-semibold">Order placed successfully!</h1>
                        <p class="text-sm text-para-light">Thank you for your order.</p>
                    </div>

                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-lg font-semibold">{{ order.order_number }}</p>
                            <p class="text-sm text-para-light">{{ formatDate(order.created_at) }}</p>
                        </div>
                        <span :class="statusClass(order.status)"
                            class="px-3 py-1 rounded-full text-xs font-medium capitalize">
                            {{ order.status == 'walkin' ? 'In Person-Order' : order.status }}
                        </span>
                    </div>

                    <div class="border-t border-gray-200 pt-4">
                        <p class="font-medium mb-2">Customer</p>
                        <p class="text-sm">{{ order.customer_name }}</p>
                    </div>

                    <div class="border-t border-gray-200 pt-4">
                        <p class="font-medium mb-2">Items</p>
                        <div v-for="item in order.items" :key="item.id" class="flex justify-between text-sm py-1">
                            <span>{{ item.product_name }} × {{ item.quantity }}</span>
                            <span>{{ formatCurrency(item.line_total) }}</span>
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

                    <NuxtLink to="/track-order"
                        class="bg-navy mt-2 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90">
                        Track another order
                    </NuxtLink>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    layout: 'guest'
});
const { publicFetch } = usePublicFetch();

const config = useRuntimeConfig().public;
const route = useRoute();

const processing = ref(false)
const order = ref(null)
const errors = reactive({ message: '' })

const statusColors = {
    pending: 'bg-yellow-100 text-yellow-800',
    confirmed: 'bg-blue-100 text-blue-800',
    processing: 'bg-indigo-100 text-indigo-800',
    shipped: 'bg-purple-100 text-purple-800',
    delivered: 'bg-green-100 text-green-800',
    cancelled: 'bg-red-100 text-red-800',
    walkin: 'bg-gray-100 text-gray-800',
}

const statusClass = (status) => statusColors[status] || 'bg-gray-100 text-gray-800'

const formatCurrency = (value) => {
    const n = Number(value);
    if (Number.isNaN(n)) return value;
    return `${config.currency} ${n.toLocaleString('en-PK', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
};

const formatDate = (value) => new Date(value).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric'
})

const fetchOrder = async () => {
    const orderId = route.params.id

    if (!orderId) {
        throw createError({
            status: 404,
            statusText: "Order not found!",
            fatal: true
        });
    }

    processing.value = true
    errors.message = ''

    try {
        const data = await publicFetch('/api/public/orders/success', {
            method: 'POST',
            body: {
                order_id: orderId
            }
        });
        if (data) {
            order.value = data;
        } else {
            throw createError({
                status: 404,
                statusText: "Order not found!",
                fatal: true
            });
        }
    } catch (e) {
        throw createError({
            status: 404,
            statusText: "Order not found!",
            fatal: true
        });
    } finally {
        processing.value = false
    }
}

onMounted(fetchOrder)
</script>