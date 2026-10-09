<template>
    <section class="w-full py-6">
        <Loading v-if="processing" message="Loading brands..." />
        <div v-else-if="brand" class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-xl sm:text-2xl font-bold text-gray-800">{{ brand.name }}</h1>
            <p v-if="brand.description" class="text-sm sm:text-base text-gray-500 mt-2">{{ brand.description }}</p>
        </div>
        <AppProducts v-if="!processing" :products="products" />
        <div v-if="!processing && hasMore" class="flex justify-center mt-6">
            <button type="button"
                class="px-6 py-2 rounded-md bg-gray-800 text-white text-sm font-medium hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="loadingMore" @click="loadMore">
                {{ loadingMore ? 'Loading...' : 'Load more' }}
            </button>
        </div>
        <AlertsError v-if="errors.message" :message="errors.message" />
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'public'
});

const PAGE_SIZE = 20;

const { publicFetch } = usePublicFetch();

const brand = ref(null);
const products = ref([]);
const processing = ref(true);
const loadingMore = ref(false);
const hasMore = ref(false);
const errors = ref({});

const route = useRoute();
const slug = route.params.slug;

const config = useRuntimeConfig();
const canonicalUrl = computed(() => `${config.public.siteUrl}/brands/${slug}`);

async function fetchPage(offset) {
    const data = await publicFetch(`/api/public/brands/${slug}?limit=${PAGE_SIZE}&offset=${offset}`);
    if (data) {
        if (!brand.value) {
            brand.value = data;
        }
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
        errors.value = { message: e.statusMessage || 'Something went wrong!' };
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
    processing.value = false;
}

useSeoMeta({
    title: () => brand.value?.name ? `${brand.value.name} Solar Products | KaleemSolarPK Multan` : 'Brands | KaleemSolarPK Multan',
    description: () => brand.value?.description || `Shop ${brand.value?.name || ''} solar panels, inverters & accessories at KaleemSolarPK Multan.`,
    keywords: () => `${brand.value?.name || ''} solar panels, ${brand.value?.name || ''} inverters, Multan, Pakistan`,
    ogTitle: () => brand.value?.name ? `${brand.value.name} Solar Products | KaleemSolarPK Multan` : 'Brands | KaleemSolarPK Multan',
    ogDescription: () => brand.value?.description || `Shop ${brand.value?.name || ''} solar panels, inverters & accessories at KaleemSolarPK Multan.`,
    ogImage: `${config.public.siteUrl}/kaleemsolar-banner.webp`,
    ogUrl: canonicalUrl.value,
    ogType: 'website',
    twitterCard: 'summary_large_image',
    twitterTitle: () => brand.value?.name ? `${brand.value.name} Solar Products | KaleemSolarPK Multan` : 'Brands | KaleemSolarPK Multan',
    twitterDescription: () => brand.value?.description || `Shop ${brand.value?.name || ''} solar panels, inverters & accessories at KaleemSolarPK Multan.`
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