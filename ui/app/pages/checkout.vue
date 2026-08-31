<template>
    <section class="w-full min-h-[80vh] px-4 py-10">
        <div class="max-w-5xl mx-auto" v-if="cartItems.length === 0 && !submitting && !orderNumber">
            <div class="text-center py-20">
                <h1 class="text-xl font-bold text-zinc-900">Your cart is empty</h1>
                <p class="text-sm text-zinc-500 mt-2">Add some products before checking out.</p>
                <NuxtLink to="/" class="inline-block mt-4 text-orange hover:underline">Back to home</NuxtLink>
            </div>
        </div>

        <div class="max-w-2xl mx-auto text-center py-20" v-else-if="orderNumber">
            <h1 class="text-2xl font-bold text-zinc-900">Order placed</h1>
            <p class="text-sm text-zinc-500 mt-2">Your order number is</p>
            <p class="text-lg font-semibold text-navy mt-1">{{ orderNumber }}</p>
            <NuxtLink to="/" class="inline-block mt-6 text-orange hover:underline">Continue shopping</NuxtLink>
        </div>

        <div class="max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-8" v-else>
            <div class="md:col-span-2 flex flex-col gap-4">
                <h1 class="text-2xl font-bold text-zinc-900">Checkout</h1>

                <div class="flex flex-col gap-3">
                    <div v-for="item in cartItems" :key="item.slug"
                        class="flex items-center gap-4 border border-gray-200 rounded-md p-3">
                        <div
                            class="w-16 h-16 rounded-md overflow-hidden bgfader flex items-center justify-center shrink-0">
                            <img v-if="item.image_url" :src="item.image_url" :alt="item.name"
                                class="w-full h-full object-contain" />
                            <span v-else class="text-zinc-400 text-xs">No image</span>
                        </div>

                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-semibold text-zinc-900 mb-1">{{ item.name }}</p>
                            <p class="text-xs text-zinc-500">SKU: {{ item.sku }}</p>
                        </div>

                        <div class="flex items-center gap-2">
                            <button type="button" @click="updateQuantity(item.slug, item.quantity - 1)"
                                class="w-7 h-7 rounded-md border border-gray-300 text-sm hover:bg-gray-50">-</button>
                            <input type="number" min="1" v-model.number="item.quantity"
                                @change="updateQuantity(item.slug, item.quantity)"
                                class="w-12 text-center text-sm border border-gray-300 rounded-md py-1" />
                            <button type="button" @click="updateQuantity(item.slug, item.quantity + 1)"
                                class="w-7 h-7 rounded-md border border-gray-300 text-sm hover:bg-gray-50">+</button>
                        </div>

                        <button type="button" @click="removeFromCart(item.slug)"
                            class="text-xs text-red-500 hover:underline shrink-0">Remove</button>
                    </div>
                </div>

                <div class="flex flex-col gap-3 mt-4">
                    <h2 class="text-lg font-semibold text-zinc-900">Delivery details</h2>

                    <p v-if="!user?.id"
                        class="text-xs text-zinc-500 bg-gray-50 border border-gray-200 rounded-md px-3 py-2">
                        Want to track your orders later? <NuxtLink to="/register" class="text-orange underline">Create
                            an account</NuxtLink> to keep them in your dashboard.
                    </p>

                    <input v-model="form.customer_name" type="text" placeholder="Full name"
                        class="border border-gray-300 rounded-md px-3 py-2 text-sm" />
                    <p v-if="errors.customer_name" class="text-xs text-red-500">{{ errors.customer_name }}</p>

                    <input v-model="form.customer_email" type="email" placeholder="Email (optional)"
                        class="border border-gray-300 rounded-md px-3 py-2 text-sm" />
                    <p v-if="errors.customer_email" class="text-xs text-red-500">{{ errors.customer_email }}</p>

                    <input v-model="form.customer_phone" type="tel" placeholder="Phone number"
                        class="border border-gray-300 rounded-md px-3 py-2 text-sm" />
                    <p v-if="errors.customer_phone" class="text-xs text-red-500">{{ errors.customer_phone }}</p>

                    <textarea v-model="form.shipping_address" rows="3" placeholder="Shipping address"
                        class="border border-gray-300 rounded-md px-3 py-2 text-sm"></textarea>
                    <p v-if="errors.shipping_address" class="text-xs text-red-500">{{ errors.shipping_address }}</p>

                    <input type="text" placeholder="Full name" disabled value="Multan (only)"
                        class="border border-gray-300 rounded-md px-3 py-2 text-sm" />

                    <textarea v-model="form.notes" rows="2" placeholder="Order notes (optional)"
                        class="border border-gray-300 rounded-md px-3 py-2 text-sm"></textarea>
                    <div class="flex flex-col">
                        <h3 class="font-semibold">
                            Important note:
                        </h3>
                        <p class="text-sm">Shipping is only available (for now) within Multan due to higher delivery
                            costs.</p>
                    </div>
                </div>
            </div>

            <div class="flex flex-col gap-4 h-fit border border-gray-200 rounded-md p-4">
                <h2 class="text-lg font-semibold text-zinc-900">Order summary</h2>

                <div class="flex flex-col gap-2 text-sm">
                    <div class="flex justify-between">
                        <span class="text-zinc-500">Subtotal</span>
                        <span>{{ formatCurrency(subtotal) }}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-zinc-500">Shipping</span>
                        <span>{{ formatCurrency(shippingAmount) }}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-zinc-500">Tax</span>
                        <span>{{ formatCurrency(taxAmount) }}</span>
                    </div>
                    <div class="flex justify-between font-semibold text-base border-t border-gray-200 pt-2 mt-1">
                        <span>Total</span>
                        <span class="text-navy">{{ formatCurrency(totalAmount) }}</span>
                    </div>
                </div>
                <p v-if="submitError" class="text-xs text-red-500">{{ submitError }}</p>

                <button type="button" :disabled="submitting" @click="placeOrder"
                    class="px-4 py-2 rounded-md text-sm font-semibold border border-navy bg-navy text-white hover:bg-orange hover:text-black hover:border-orange transition-colors disabled:opacity-40 disabled:cursor-not-allowed">
                    {{ submitting ? 'Placing order...' : 'Place order' }}
                </button>
                <p class="text-sm mb-2">By placing your order you agree to our <NuxtLink class="text-orange underline"
                        to="/terms-of-service" target="_blank">Terms of
                        service</NuxtLink>, <NuxtLink class="text-orange underline" to="/privacy-policy"
                        target="_blank">privacy</NuxtLink> & <NuxtLink class="text-orange underline" to="/return-policy"
                        target="_blank">refund policy</NuxtLink>
                </p>
                <div class="my-3">
                    <NuxtTurnstile ref="turnstile" v-model="ct_token" />
                </div>
            </div>
        </div>
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'guest'
});
const { user } = useAuthUser();
const { cartItems, updateQuantity, removeFromCart, clearCart } = useCart();
const { publicFetch } = usePublicFetch();

const submitting = ref(false);
const submitError = ref('');
const orderNumber = ref('');
const ct_token = ref("");

const form = reactive({
    customer_name: '',
    customer_email: '',
    customer_phone: '',
    shipping_address: '',
    notes: ''
});

const errors = reactive({
    customer_name: '',
    customer_email: '',
    customer_phone: '',
    shipping_address: ''
});

const taxRate = 0;
const flatShipping = 0;

const subtotal = computed(() =>
    cartItems.value.reduce((sum, item) => sum + (Number(item.unit_price) || 0) * item.quantity, 0)
);

const taxAmount = computed(() => subtotal.value * taxRate);
const shippingAmount = computed(() => (subtotal.value > 0 ? flatShipping : 0));
const totalAmount = computed(() => subtotal.value + taxAmount.value + shippingAmount.value);

function formatCurrency(amount) {
    const currency = useRuntimeConfig().public.currency;
    return new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: currency
    }).format(Number(amount));
}


function validate() {
    errors.customer_name = form.customer_name.trim() ? '' : 'Name is required';
    errors.customer_phone = form.customer_phone.trim() ? '' : 'Phone number is required';
    errors.shipping_address = form.shipping_address.trim() ? '' : 'Shipping address is required';
    errors.customer_email = form.customer_email && !/^\S+@\S+\.\S+$/.test(form.customer_email)
        ? 'Enter a valid email'
        : '';

    return !errors.customer_name && !errors.customer_phone && !errors.shipping_address && !errors.customer_email;
}

async function placeOrder() {
    submitError.value = '';

    if (!validate()) return;
    if (cartItems.value.length === 0) return;

    submitting.value = true;

    try {
        const payload = {
            ct_token: ct_token.value,
            user_id: user.value?.id ? user.value.id : null,
            customer_name: form.customer_name.trim(),
            customer_email: form.customer_email.trim() || null,
            customer_phone: form.customer_phone.trim(),
            shipping_address: form.shipping_address.trim(),
            notes: form.notes.trim() || null,
            items: cartItems.value.map(item => ({
                slug: item.slug,
                sku: item.sku,
                quantity: item.quantity,
            }))
        };

        const data = await publicFetch('/api/public/orders', {
            method: 'POST',
            body: payload
        });
        if (data) {
            orderNumber.value = data.order_number;
            clearCart();
            await navigateTo(`/order/success/${data.order_number}`)
        } else {
            throw createError({
                statusText: 'Something went wrong!',
                fatal: true
            });
        }
    } catch (e) {
        submitError.value = e.statusMessage || e.message || 'Something went wrong placing your order.';
    } finally {
        submitting.value = false;
    }
}
</script>