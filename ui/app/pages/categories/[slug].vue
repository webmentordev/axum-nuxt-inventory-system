<template>
    <section class="w-full py-6">
        <Loading v-if="processing" message="Loading category..." />
        <div v-else-if="category" class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-xl sm:text-2xl font-bold text-gray-800">{{ category.name }}</h1>
            <p v-if="category.description" class="text-sm sm:text-base text-gray-500 mt-2">{{ category.description }}
            </p>
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

const category = ref(null);
const products = ref([]);
const processing = ref(true);
const loadingMore = ref(false);
const hasMore = ref(false);
const errors = ref({});

const route = useRoute();
const slug = route.params.slug;

const config = useRuntimeConfig();
const canonicalUrl = computed(() => `${config.public.siteUrl}/categories/${slug}`);

async function fetchPage(offset) {
    const data = await publicFetch(`/api/public/categories/${slug}?limit=${PAGE_SIZE}&offset=${offset}`);
    if (data) {
        if (!category.value) {
            category.value = data;
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
    title: () => category.value?.name ? `${category.value.name} | KaleemSolarPK Multan` : 'Categories | KaleemSolarPK Multan',
    description: () => category.value?.description || `Shop ${category.value?.name || ''} at KaleemSolarPK Multan. A-Grade solar panels, inverters & accessories.`,
    keywords: () => `${category.value?.name || ''}, solar panels, solar inverters, Multan, Pakistan`,
    ogTitle: () => category.value?.name ? `${category.value.name} | KaleemSolarPK Multan` : 'Categories | KaleemSolarPK Multan',
    ogDescription: () => category.value?.description || `Shop ${category.value?.name || ''} at KaleemSolarPK Multan. A-Grade solar panels, inverters & accessories.`,
    ogImage: `${config.public.siteUrl}/kaleemsolar-banner.webp`,
    ogUrl: canonicalUrl.value,
    ogType: 'website',
    twitterCard: 'summary_large_image',
    twitterTitle: () => category.value?.name ? `${category.value.name} | KaleemSolarPK Multan` : 'Categories | KaleemSolarPK Multan',
    twitterDescription: () => category.value?.description || `Shop ${category.value?.name || ''} at KaleemSolarPK Multan. A-Grade solar panels, inverters & accessories.`
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