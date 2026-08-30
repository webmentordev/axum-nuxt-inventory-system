<template>
    <nav class="flex flex-col justify-between w-full p-2 h-full">
        <div class="w-full mb-3 pb-3 border-b border-dark-400">
            <img src="/kaleem-solar-logo-t-4.png" width="160px" class="justify-center flex w-fit">
        </div>
        <div class="w-full flex flex-col gap-1 h-full">
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
            <NuxtLink to="/admin/users"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="mdi:account-outline" size="20" />
                <strong class="ml-1">Users</strong>
            </NuxtLink>
            <NuxtLink to="/admin/products"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="game-icons:cargo-ship" size="20" />
                <strong class="ml-1">Products</strong>
            </NuxtLink>
            <NuxtLink to="/admin/seo"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="streamline-freehand:programming-search" size="20" />
                <strong class="ml-1">Product SEO</strong>
            </NuxtLink>
            <NuxtLink to="/admin/brands"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="mdi:tag-outline" size="20" />
                <strong class="ml-1">Brands</strong>
            </NuxtLink>
            <NuxtLink to="/admin/categories"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="bxs:category-alt" size="20" />
                <strong class="ml-1">Categories</strong>
            </NuxtLink>
            <NuxtLink to="/admin/sub-categories"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="material-symbols:filter-list-rounded" size="20" />
                <strong class="ml-1">Sub categories</strong>
            </NuxtLink>
            <NuxtLink to="/admin/policies"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="material-symbols:policy-outline-rounded" size="20" />
                <strong class="ml-1">Policies</strong>
            </NuxtLink>
            <NuxtLink to="/admin/uploads"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="tabler:image-generation" size="20" />
                <strong class="ml-1">Uploads</strong>
            </NuxtLink>
            <NuxtLink to="/admin/contacts"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="tabler:mail" size="20" />
                <strong class="ml-1">Contacts</strong>
            </NuxtLink>
            <NuxtLink to="/admin/report"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon name="tabler:graph" size="20" />
                <strong class="ml-1">Report</strong>
            </NuxtLink>
        </div>
        <AdminButton icon="tabler:door-exit" class="py-3" @click="logout">Logout</AdminButton>
    </nav>
</template>

<script setup lang="js">
const { removeToken } = useAuthToken();
const { removeUser } = useAuthUser();

async function logout() {
    removeToken();
    removeUser();
    await navigateTo('/login');
}
</script>