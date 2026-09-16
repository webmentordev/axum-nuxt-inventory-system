<template>
    <section class="w-full py-6">
        <Loading v-if="pending" message="Loading products..." />
        <div class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-xl sm:text-2xl font-bold text-gray-800">Products</h1>
            <p class="text-sm sm:text-base text-gray-500 mt-2">Our solar products listing</p>
        </div>
        <AppProducts v-if="!pending && products.length > 0" :products="products" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    layout: 'public'
});

const { publicFetch } = usePublicFetch();

const products = ref([]);
const processing = ref(true);

const config = useRuntimeConfig();
const canonicalUrl = computed(() => `${config.public.siteUrl}/products`);

try {
    const data = await publicFetch('/api/public/products');
    if (data) {
        products.value = data;
    }
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