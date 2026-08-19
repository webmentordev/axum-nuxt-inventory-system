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
                <NuxtLink to="/" class="hover:text-orange">Blog</NuxtLink>
                <NuxtLink to="/" class="hover:text-orange">Contact Us</NuxtLink>
            </nav>

            <div class="flex items-center gap-5 text-xl text-navy">
                <button>⌕</button>
                <button>🛒</button>
            </div>
        </div>
    </nav>
</template>