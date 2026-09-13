<template>
    <nav class="flex flex-col justify-between w-full p-2 h-full">
        <div class="w-full mb-3 pb-3 border-b border-dark-400 flex items-center justify-between">
            <img src="/kaleem-solar-logo-t-4.png" width="160px" class="justify-center flex w-fit">
            <button class="lg:hidden text-zinc-300" @click="isOpen = false">
                <Icon name="mdi:close" size="24" />
            </button>
        </div>
        <div class="w-full flex flex-col gap-1 h-full overflow-y-auto themed-scroll">
            <NuxtLink v-for="item in links" :key="item.to" :to="item.to" @click="isOpen = false"
                class="w-full flex items-center px-3 py-2 rounded-md text-sm font-semibold text-zinc-300 transition-colors hover:text-white hover:bg-dark-300"
                active-class="!text-lime-main !bg-lime-bg !border !border-lime-main/30">
                <Icon :name="item.icon" size="20" />
                <strong class="ml-1">{{ item.label }}</strong>
            </NuxtLink>
        </div>
        <AdminButton icon="tabler:door-exit" class="py-3" @click="logout">Logout</AdminButton>
    </nav>
</template>

<script setup lang="js">
const isOpen = useState('admin-nav-open', () => false);
const { removeToken } = useAuthToken();
const { removeUser } = useAuthUser();

const links = [
    { to: '/', icon: 'mdi-light:home', label: 'Home' },
    { to: '/admin/dashboard', icon: 'mdi-light:view-dashboard', label: 'Dashboard' },
    { to: '/admin/users', icon: 'mdi:account-outline', label: 'Users' },
    { to: '/admin/products', icon: 'game-icons:cargo-ship', label: 'Products' },
    { to: '/admin/seo', icon: 'streamline-ultimate:seo-search-graph-bold', label: 'Product SEO' },
    { to: '/admin/brands', icon: 'mdi:tag-outline', label: 'Brands' },
    { to: '/admin/categories', icon: 'bxs:category-alt', label: 'Categories' },
    { to: '/admin/sub-categories', icon: 'material-symbols:filter-list-rounded', label: 'Sub categories' },
    { to: '/admin/orders', icon: 'carbon:delivery-parcel', label: 'Orders' },
    { to: '/admin/barcodes', icon: 'si:barcode-scan-fill', label: 'Barcodes' },
    { to: '/admin/policies', icon: 'carbon:policy', label: 'Policies' },
    { to: '/admin/uploads', icon: 'ic:round-upload', label: 'Uploads' },
    { to: '/admin/contacts', icon: 'tabler:mail', label: 'Contacts' },
    { to: '/admin/report', icon: 'tabler:graph', label: 'Report' },
    { to: '/admin/audit-logs', icon: 'ant-design:audit-outlined', label: 'Audit logs' },
];

async function logout() {
    removeToken();
    removeUser();
    await navigateTo('/login');
}
</script>