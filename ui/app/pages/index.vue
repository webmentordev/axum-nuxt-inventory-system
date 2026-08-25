<template>
    <section class="w-full">
        <AppProducts v-if="!processing" :products="products" />
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