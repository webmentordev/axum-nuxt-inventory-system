<template>
    <section class="w-full">
        <div v-if="products.length" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 py-3">
            <ProductsCard v-for="product in products" :product="product" :key="product.id" />
        </div>
        <p v-else class="text-gray-500">No products found.</p>
    </section>
</template>

<script setup lang="js">
defineProps({
    products: {
        type: Array,
        default: () => []
    }
});
</script>