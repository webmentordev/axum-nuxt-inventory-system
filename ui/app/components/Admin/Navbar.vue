<template>
    <nav class="w-full flex flex-col p-2 gap-1">
        <NuxtLink to="/"
            class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
            active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
            <Icon name="mdi-light:home" size="20" />
            <strong class="ml-1">Home</strong>
        </NuxtLink>
        <NuxtLink to="/admin/dashboard"
            class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
            active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
            <Icon name="mdi-light:view-dashboard" size="20" />
            <strong class="ml-1">Dashboard</strong>
        </NuxtLink>
        <NuxtLink to="/admin/products"
            class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
            active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
            <Icon name="game-icons:cargo-ship" size="20" />
            <strong class="ml-1">Products</strong>
        </NuxtLink>
        <NuxtLink to="/admin/categories"
            class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
            active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
            <Icon name="bxs:category-alt" size="20" />
            <strong class="ml-1">Categories</strong>
        </NuxtLink>
    </nav>
</template>