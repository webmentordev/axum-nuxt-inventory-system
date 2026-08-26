<template>
    <section class="w-full">
        <Loading v-if="processing" message="Loading category..." />
        <div v-else-if="category" class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-2xl font-bold text-gray-800">{{ category.name }}</h1>
            <p v-if="category.description" class="text-gray-500 mt-2">{{ category.description }}</p>
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
</script>