<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Create order</h1>
            <p class="text-sm text-zinc-500 mt-1">Add a new order for a customer.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Status</label>
                    <select v-model="status"
                        class="w-full rounded-md bg-dark-200 border border-dark-300 px-3 py-2 text-sm text-zinc-200 focus:outline-none focus:border-lime-main">
                        <option value="pending">Pending</option>
                        <option value="confirmed">Confirmed</option>
                        <option value="processing">Processing</option>
                        <option value="shipped">Shipped</option>
                        <option value="delivered">Delivered</option>
                        <option value="cancelled">Cancelled</option>
                        <option value="walkin">Walk-in</option>
                    </select>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Customer name</label>
                    <AdminInput v-model="customerName" placeholder="e.g. John Doe" />
                    <p v-if="errors.customerName" class="text-xs text-red-400 mt-1">{{ errors.customerName }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Email</label>
                    <AdminInput v-model="customerEmail" placeholder="john@example.com" />
                    <p v-if="errors.customerEmail" class="text-xs text-red-400 mt-1">{{ errors.customerEmail }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Phone</label>
                    <AdminInput v-model="customerPhone" placeholder="+1 234 567 890" />
                    <p v-if="errors.customerPhone" class="text-xs text-red-400 mt-1">{{ errors.customerPhone }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Shipping address</label>
                    <AdminTextarea v-model="shippingAddress" placeholder="Street, city, state, zip..." rows="3" />
                    <p v-if="errors.shippingAddress" class="text-xs text-red-400 mt-1">{{ errors.shippingAddress }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Notes</label>
                    <AdminTextarea v-model="notes" placeholder="Optional notes..." rows="4" />
                    <p v-if="errors.notes" class="text-xs text-red-400 mt-1">{{ errors.notes }}</p>
                </div>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Create order
                </button>
            </form>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const customerName = ref('');
const customerEmail = ref('');
const customerPhone = ref('');
const shippingAddress = ref('');
const notes = ref('');

const status = ref('walkin');

const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

function validate() {
    errors.value = {};
    if (!customerName.value.trim()) {
        errors.value.customerName = 'Customer name is required.';
    }
    return Object.keys(errors.value).length === 0;
}

async function handleSubmit() {
    if (!validate()) return;

    statusType.value = 'loading';
    statusMessage.value = 'Creating order...';
    showStatus.value = true;

    try {
        const order = await authFetch('/api/admin/orders', {
            method: 'POST',
            body: {
                customer_name: customerName.value.trim(),
                customer_email: customerEmail.value.trim() || null,
                customer_phone: customerPhone.value.trim() || null,
                shipping_address: shippingAddress.value.trim() || null,
                notes: notes.value.trim() || null,
                status: status.value
            }
        });

        statusType.value = 'success';
        statusMessage.value = 'Order created.';

        setTimeout(() => {
            navigateTo(`/admin/orders/${order.id}/items`);
        }, 600);
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to create order.';
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}
</script>