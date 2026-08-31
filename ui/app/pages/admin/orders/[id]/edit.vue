<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Edit order</h1>
            <p class="text-sm text-zinc-500 mt-1">Update the order's details.</p>

            <form v-if="loaded" @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Status</label>
                    <AdminSelect v-model="status" :options="statusOptions" placeholder="Select a status" />
                    <p v-if="errors.status" class="text-xs text-red-400 mt-1">{{ errors.status }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Customer account</label>
                    <AdminSelect v-model="userId" :options="userOptions"
                        :placeholder="usersLoading ? 'Loading customers...' : 'No linked account'" />
                    <p class="text-xs text-zinc-500 mt-1">Optional — link this order to a registered customer.</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Customer name</label>
                    <AdminInput v-model="customerName" placeholder="e.g. John Doe" />
                    <p v-if="errors.customerName" class="text-xs text-red-400 mt-1">{{ errors.customerName }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Email</label>
                    <AdminInput v-model="customerEmail" placeholder="john@example.com" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Phone</label>
                    <AdminInput v-model="customerPhone" placeholder="+1 234 567 890" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Shipping address</label>
                    <AdminTextarea v-model="shippingAddress" placeholder="Street, city, state, zip..." rows="3" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Notes</label>
                    <AdminTextarea v-model="notes" placeholder="Optional notes..." rows="4" />
                </div>

                <button type="submit" :disabled="submitting"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit disabled:opacity-50 disabled:cursor-not-allowed">
                    {{ submitting ? 'Saving...' : 'Save changes' }}
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
const route = useRoute();

const orderUuid = route.params.id;

const users = ref([]);
const usersLoading = ref(true);

const userId = ref(null);
const customerName = ref('');
const customerEmail = ref('');
const customerPhone = ref('');
const shippingAddress = ref('');
const status = ref('pending');
const notes = ref('');

const loaded = ref(false);
const errors = ref({});
const submitting = ref(false);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const statusOptions = [
    { label: 'Pending', value: 'pending' },
    { label: 'Confirmed', value: 'confirmed' },
    { label: 'Processing', value: 'processing' },
    { label: 'Shipped', value: 'shipped' },
    { label: 'Delivered', value: 'delivered' },
    { label: 'Cancelled', value: 'cancelled' },
    { label: 'Walk-in', value: 'walkin' },
    { label: 'Walk-in Completed', value: 'walkin_completed' }
];

const userOptions = computed(() =>
    users.value.map((user) => ({
        label: user.is_active ? `${user.name} — ${user.email}` : `${user.name} — ${user.email} (Inactive)`,
        value: user.id
    }))
);

async function fetchUsers() {
    usersLoading.value = true;
    try {
        const data = await authFetch('/api/admin/users');
        if (data) {
            users.value = data;
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Failed to load customers.';
    } finally {
        usersLoading.value = false;
    }
}

async function fetchOrder() {
    try {
        const data = await authFetch(`/api/admin/orders/${orderUuid}`);
        if (data) {
            userId.value = data.user_id || null;
            customerName.value = data.customer_name || '';
            customerEmail.value = data.customer_email || '';
            customerPhone.value = data.customer_phone || '';
            shippingAddress.value = data.shipping_address || '';
            status.value = data.status || 'pending';
            notes.value = data.notes || '';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to load order.';
        showStatus.value = true;
    } finally {
        loaded.value = true;
    }
}

function validate() {
    errors.value = {};
    if (!customerName.value.trim()) {
        errors.value.customerName = 'Customer name is required.';
    }
    return Object.keys(errors.value).length === 0;
}

async function handleSubmit() {
    if (!validate()) return;

    submitting.value = true;
    statusType.value = 'loading';
    statusMessage.value = 'Saving order...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/orders/${orderUuid}`, {
            method: 'PATCH',
            body: {
                user_id: userId.value || null,
                customer_name: customerName.value.trim(),
                customer_email: customerEmail.value.trim() || null,
                customer_phone: customerPhone.value.trim() || null,
                shipping_address: shippingAddress.value.trim() || null,
                status: status.value,
                notes: notes.value.trim() || null
            }
        });

        statusType.value = 'success';
        statusMessage.value = 'Order updated.';

        setTimeout(() => {
            navigateTo(`/admin/orders/${orderUuid}/items`);
        }, 600);
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update order.';
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    } finally {
        submitting.value = false;
    }
}

await Promise.all([fetchUsers(), fetchOrder()]);
</script>