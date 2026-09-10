<template>
    <nav class="border-b border-gray-300 bg-white w-full">
        <div class="mx-auto flex max-w-7xl items-center justify-between py-2 px-4 gap-3">
            <NuxtLink to="/" class="flex items-center shrink-0">
                <img src="/kaleem-solar-logo-t-2.png" class="w-24 sm:w-30">
            </NuxtLink>

            <div class="hidden md:block flex-1">
                <ProductsSearch />
            </div>

            <div class="flex items-center gap-3 sm:gap-5 text-xl text-navy">
                <button class="md:hidden h-fit flex items-center" @click="mobileSearchOpen = !mobileSearchOpen">
                    <Icon name="ic:outline-search" size="24px" />
                </button>

                <div class="relative" ref="cartDropdownRef">
                    <button @click="cartDropdownOpen = !cartDropdownOpen" class="relative">
                        🛒
                        <span v-if="cartCount > 0"
                            class="absolute -top-2 -right-2 flex h-4 min-w-4 items-center justify-center rounded-full bg-orange px-1 text-[10px] font-semibold text-black">
                            {{ cartCount }}
                        </span>
                    </button>

                    <div v-if="cartDropdownOpen"
                        class="absolute right-0 mt-2 w-[calc(100vw-2rem)] max-w-80 rounded-md border border-gray-200 bg-white py-2 shadow-lg text-sm z-50">
                        <div v-if="cartItems.length === 0" class="px-4 py-6 text-center text-zinc-500">
                            Your cart is empty
                        </div>

                        <div v-else class="flex flex-col">
                            <div class="max-h-72 overflow-y-auto flex flex-col divide-y divide-gray-100">
                                <div v-for="item in cartItems" :key="item.slug"
                                    class="flex items-center gap-3 px-4 py-2">
                                    <div
                                        class="w-10 h-10 rounded-md overflow-hidden bgfader flex items-center justify-center shrink-0">
                                        <img v-if="item.image_url" :src="item.image_url" :alt="item.name"
                                            class="w-full h-full object-contain" />
                                        <span v-else class="text-zinc-400 text-[10px]">No image</span>
                                    </div>

                                    <div class="flex-1 min-w-0">
                                        <p class="text-xs font-semibold text-zinc-900 truncate">{{ item.name }}</p>
                                        <p class="text-[11px] text-zinc-500">
                                            {{ item.quantity }} x Rs. {{ formatCurrency(item.unit_price) }}
                                        </p>
                                    </div>

                                    <button @click="removeFromCart(item.slug)"
                                        class="text-[11px] text-red-500 hover:underline shrink-0">
                                        Remove
                                    </button>
                                </div>
                            </div>

                            <div class="flex items-center justify-between px-4 pt-2 pb-1 font-semibold text-navy">
                                <span>Subtotal</span>
                                <span>{{ formatCurrency(cartSubtotal) }}</span>
                            </div>

                            <NuxtLink to="/checkout" @click="cartDropdownOpen = false"
                                class="block mx-4 mt-2 text-center px-3 py-2 rounded-md text-xs font-semibold border border-navy bg-navy text-white hover:bg-orange hover:text-black hover:border-orange transition-colors">
                                Go to checkout
                            </NuxtLink>
                        </div>
                    </div>
                </div>

                <div v-if="user" class="relative" ref="dropdownRef">
                    <button @click="dropdownOpen = !dropdownOpen"
                        class="flex items-center gap-1 text-sm font-medium text-navy">
                        <span class="hidden sm:inline">{{ user.name }}</span>
                        <span class="sm:hidden text-xl">👤</span>
                        <span class="text-xs hidden sm:inline">▾</span>
                    </button>
                    <div v-if="dropdownOpen"
                        class="absolute right-0 mt-2 w-40 rounded-md border border-gray-200 bg-white py-1 shadow-lg z-40">
                        <NuxtLink to="/user/profile" class="block px-4 py-2 text-sm text-navy hover:bg-gray-100"
                            @click="dropdownOpen = false">Profile</NuxtLink>
                        <NuxtLink v-if="user.is_admin" to="/admin/dashboard"
                            class="block px-4 py-2 text-sm text-navy hover:bg-gray-100" @click="dropdownOpen = false">
                            Dashboard</NuxtLink>
                        <button @click="handleLogout"
                            class="block w-full px-4 py-2 text-left text-sm text-navy hover:bg-gray-100">Logout</button>
                    </div>
                </div>
                <NuxtLink v-else to="/login" class="text-sm font-medium hover:text-orange">Login</NuxtLink>
            </div>
        </div>

        <div v-if="mobileSearchOpen" class="md:hidden px-4 pb-3">
            <ProductsSearch />
        </div>
    </nav>
</template>

<script setup>
const { user } = useAuthUser();
const { removeToken } = useAuthToken();
const { removeUser } = useAuthUser();
const dropdownOpen = ref(false);
const dropdownRef = ref(null);

const { cartItems, cartCount, removeFromCart } = useCart();
const cartDropdownOpen = ref(false);
const cartDropdownRef = ref(null);
const mobileSearchOpen = ref(false);

const cartSubtotal = computed(() =>
    cartItems.value.reduce((sum, item) => sum + (Number(item.unit_price) || 0) * item.quantity, 0)
);

function formatCurrency(amount) {
    const currency = useRuntimeConfig().public.currency;
    return new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: currency
    }).format(Number(amount));
}

const handleLogout = async () => {
    dropdownOpen.value = false;
    removeToken();
    removeUser();
    await navigateTo('/login');
};

onClickOutside(dropdownRef, () => {
    dropdownOpen.value = false;
});

onClickOutside(cartDropdownRef, () => {
    cartDropdownOpen.value = false;
});
</script>