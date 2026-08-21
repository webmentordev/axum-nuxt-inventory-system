<template>
    <nav class="border-b border-gray-300 bg-white sticky z-40 top-0 left-0 w-full">
        <div class="mx-auto flex max-w-7xl items-center justify-between px-4 py-4">
            <NuxtLink to="/" class="flex items-center">
                <img src="/kaleem-solar-logo-t-2.png" width="120px">
            </NuxtLink>
            <nav class="hidden items-center gap-7 text-sm font-medium lg:flex">
                <NuxtLink to="/" class="text-orange">Home</NuxtLink>
                <NuxtLink to="/" class="hover:text-orange">Solar Panels</NuxtLink>
                <NuxtLink to="/" class="hover:text-orange">Inverters</NuxtLink>
                <NuxtLink to="/" class="hover:text-orange">Batteries</NuxtLink>
                <NuxtLink to="/" class="hover:text-orange">Solar Structures</NuxtLink>
                <NuxtLink to="/" class="hover:text-orange">Cables & Accessories</NuxtLink>
                <NuxtLink to="/contact-us" class="hover:text-orange">Contact Us</NuxtLink>
            </nav>

            <div class="flex items-center gap-5 text-xl text-navy">
                <button>⌕</button>
                <button>🛒</button>

                <div v-if="user" class="relative" ref="dropdownRef">
                    <button @click="dropdownOpen = !dropdownOpen"
                        class="flex items-center gap-1 text-sm font-medium text-navy">
                        <span>{{ user.name }}</span>
                        <span class="text-xs">▾</span>
                    </button>
                    <div v-if="dropdownOpen"
                        class="absolute right-0 mt-2 w-40 rounded-md border border-gray-200 bg-white py-1 shadow-lg">
                        <NuxtLink to="/user/profile" class="block px-4 py-2 text-sm text-navy hover:bg-gray-100"
                            @click="dropdownOpen = false">Profile</NuxtLink>
                        <button @click="handleLogout"
                            class="block w-full px-4 py-2 text-left text-sm text-navy hover:bg-gray-100">Logout</button>
                    </div>
                </div>
                <NuxtLink v-else to="/login" class="text-sm font-medium hover:text-orange">Login</NuxtLink>
            </div>
        </div>
    </nav>
</template>

<script setup>
const { user } = useAuthUser();
const { removeToken } = useAuthToken();
const { removeUser } = useAuthUser();
const dropdownOpen = ref(false);
const dropdownRef = ref(null);

const handleLogout = async () => {
    dropdownOpen.value = false;
    removeToken();
    removeUser();
    await navigateTo('/login');
};

onClickOutside(dropdownRef, () => {
    dropdownOpen.value = false;
});
</script>