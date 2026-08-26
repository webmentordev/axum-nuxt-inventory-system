<template>
    <section class="w-full">
        <Loading v-if="processing" message="Loading brands..." />
        <div v-else-if="brand" class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-2xl font-bold text-gray-800">{{ brand.name }}</h1>
            <p v-if="brand.description" class="text-gray-500 mt-2">{{ brand.description }}</p>
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

const brand = ref(null);
const products = ref([]);
const processing = ref(true);
const errors = ref({});

const route = useRoute();
const slug = route.params.slug;

try {
    const data = await publicFetch('/api/public/brands/' + slug);
    if (data) {
        brand.value = data;
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
</script>