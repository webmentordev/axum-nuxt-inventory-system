<template>
    <section class="w-full">
        <Loading v-if="pending" message="Loading products..." />
        <div class="pb-3 border-b border-gray-200 mb-2">
            <h1 class="text-2xl font-bold text-gray-800">Products</h1>
            <p class="text-gray-500 mt-2">Our solar products listing</p>
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
</script>