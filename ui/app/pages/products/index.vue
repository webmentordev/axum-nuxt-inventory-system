<template>
    <section class="w-full py-6">
        <Loading v-if="pending" message="Loading products..." />
        <div class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-xl sm:text-2xl font-bold text-gray-800">Products</h1>
            <p class="text-sm sm:text-base text-gray-500 mt-2">Our solar products listing</p>
        </div>
        <AppProducts v-if="!pending && products.length > 0" :products="products" />
        <div v-if="!pending && hasMore" class="flex justify-center mt-6">
            <button type="button"
                class="px-6 py-2 rounded-md bg-gray-800 text-white text-sm font-medium hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="loadingMore" @click="loadMore">
                {{ loadingMore ? 'Loading...' : 'Load more' }}
            </button>
        </div>
    </section>
</template>

<script setup lang="js">
definePageMeta({
    layout: 'public'
});

const PAGE_SIZE = 20;

const { publicFetch } = usePublicFetch();

const products = ref([]);
const pending = ref(true);
const loadingMore = ref(false);
const hasMore = ref(false);

const config = useRuntimeConfig();
const canonicalUrl = computed(() => `${config.public.siteUrl}/products`);

async function fetchPage(offset) {
    const data = await publicFetch(`/api/public/products?limit=${PAGE_SIZE}&offset=${offset}`);
    if (data) {
        products.value.push(...data.products);
        hasMore.value = data.has_more;
    }
}

async function loadMore() {
    if (loadingMore.value) return;
    loadingMore.value = true;
    try {
        await fetchPage(products.value.length);
    } catch (e) {
        console.error(e);
    } finally {
        loadingMore.value = false;
    }
}

try {
    await fetchPage(0);
} catch (e) {
    throw createError({
        status: e.statusCode || 500,
        statusText: e.statusMessage || 'Something went wrong!',
        fatal: true
    });
} finally {
    pending.value = false;
}

useSeoMeta({
    title: 'Solar Products | KaleemSolarPK Multan',
    description: 'Browse our full range of A-Grade solar panels, inverters, batteries & accessories at the best prices in Multan, Pakistan.',
    keywords: 'solar panels, solar inverters, solar batteries, solar accessories, Multan, Pakistan',
    ogTitle: 'Solar Products | KaleemSolarPK Multan',
    ogDescription: 'Browse our full range of A-Grade solar panels, inverters, batteries & accessories at the best prices in Multan, Pakistan.',
    ogImage: `${config.public.siteUrl}/kaleemsolar-banner.webp`,
    ogUrl: canonicalUrl.value,
    ogType: 'website',
    twitterCard: 'summary_large_image',
    twitterTitle: 'Solar Products | KaleemSolarPK Multan',
    twitterDescription: 'Browse our full range of A-Grade solar panels, inverters, batteries & accessories at the best prices in Multan, Pakistan.'
});

useHead({
    link: [
        {
            rel: 'canonical',
            href: canonicalUrl.value
        }
    ]
});
</script>