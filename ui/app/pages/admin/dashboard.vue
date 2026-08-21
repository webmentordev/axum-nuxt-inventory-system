<template>
    <section class="h-full w-full p-6">
        <div class="mb-6">
            <h1 class="text-xl font-bold text-white">Dashboard</h1>
            <p class="text-sm text-zinc-500 mt-1">Overview of your store data.</p>
        </div>

        <div v-if="loading" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <div v-for="n in 10" :key="n" class="border border-dark-300 rounded-xl bg-dark-100 p-4 h-24 animate-pulse">
            </div>
        </div>

        <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <div v-for="card in cards" :key="card.label"
                class="border border-dark-300 rounded-xl bg-dark-100 p-4 flex items-start justify-between hover:border-dark-strong transition-colors">
                <div>
                    <p class="text-sm text-zinc-500">{{ card.label }}</p>
                    <p class="text-2xl font-bold text-white mt-1">{{ card.value.toLocaleString() }}</p>
                </div>
                <div class="w-10 h-10 rounded-lg flex items-center justify-center shrink-0 bg-lime-bg">
                    <Icon :name="card.icon" size="20" class="text-lime-main" />
                </div>
            </div>
        </div>

        <div v-if="errorMessage"
            class="mt-4 px-4 py-3 rounded-xl border border-red-500/30 bg-dark-200 text-sm text-red-400">
            {{ errorMessage }}
        </div>
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const stats = ref(null);
const loading = ref(true);
const errorMessage = ref('');

const cards = computed(() => {
    if (!stats.value) return [];

    return [
        {
            label: 'Products',
            value: stats.value.products_count,
            icon: 'mdi:cube-outline'
        },
        {
            label: 'Orders',
            value: stats.value.orders_count,
            icon: 'mdi:cart-outline'
        },
        {
            label: 'Categories',
            value: stats.value.categories_count,
            icon: 'mdi:shape-outline'
        },
        {
            label: 'Sub-categories',
            value: stats.value.sub_categories_count,
            icon: 'mdi:shape-plus-outline'
        },
        {
            label: 'Brands',
            value: stats.value.brands_count,
            icon: 'mdi:tag-outline'
        },
        {
            label: 'Images',
            value: stats.value.images_count,
            icon: 'mdi:image-outline'
        },
        {
            label: 'Barcodes',
            value: stats.value.barcodes_count,
            icon: 'mdi:barcode'
        },
        {
            label: 'Contacts',
            value: stats.value.contacts_count,
            icon: 'mdi:email-outline'
        },
        {
            label: 'Users',
            value: stats.value.users_count,
            icon: 'mdi:account-outline'
        },
        {
            label: 'Admin Users',
            value: stats.value.admin_users_count,
            icon: 'mdi:shield-account-outline'
        }
    ];
});

async function fetchStats() {
    loading.value = true;
    errorMessage.value = '';
    try {
        const data = await authFetch('/api/stats');
        if (data) {
            stats.value = data;
        }
    } catch (e) {
        errorMessage.value = e.statusMessage || 'Failed to load stats.';
    } finally {
        loading.value = false;
    }
}

await fetchStats();
</script>