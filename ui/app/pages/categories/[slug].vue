<template>
    <section class="w-full py-6">
        <Loading v-if="processing" message="Loading category..." />
        <div v-else-if="category" class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-xl sm:text-2xl font-bold text-gray-800">{{ category.name }}</h1>
            <p v-if="category.description" class="text-sm sm:text-base text-gray-500 mt-2">{{ category.description }}
            </p>
        </div>
        <AppProducts v-if="!processing" :products="products" />
        <AlertsError v-if="errors.message" :message="errors.message" />
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'public'
});

const { publicFetch } = usePublicFetch();

const category = ref(null);
const products = ref([]);
const processing = ref(true);
const errors = ref({});

const route = useRoute();
const slug = route.params.slug;

const config = useRuntimeConfig();
const canonicalUrl = computed(() => `${config.public.siteUrl}/categories/${slug}`);

try {
    const data = await publicFetch('/api/public/categories/' + slug);
    if (data) {
        category.value = data;
        products.value = data.products;
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