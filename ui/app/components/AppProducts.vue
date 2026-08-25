<template>
    <section class="w-full p-4">
        <h1 class="text-2xl font-bold mb-4">Products</h1>

        <div v-if="products.length" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
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