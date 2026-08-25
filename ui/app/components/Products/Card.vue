<template>
    <NuxtLink :to='`/products/${product.slug}`' class="overflow-hidden flex flex-col">
        <img :src="product.image_url" :alt="product.name" class="w-full h-40 object-cover bg-gray-100" loading="lazy" />
        <div class="p-3 flex flex-col flex-1">
            <h2 class="font-semibold text-sm line-clamp-2">{{ product.name }}</h2>

            <p class="text-xs text-gray-500 mt-1">
                {{ product.brand || 'No brand' }}
                <span v-if="product.model"> · {{ product.model }}</span>
            </p>

            <div class="mt-auto pt-3 flex items-center justify-between">
                <span class="font-bold text-lg">
                    {{ config.currency }} {{ Number(product.selling_price).toFixed(2) }}
                </span>
                <span class="text-xs px-2 py-1 rounded-full" :class="product.in_stock
                    ? 'bg-green-100 text-green-700'
                    : 'bg-red-100 text-red-700'">
                    {{ product.in_stock ? 'In stock' : 'Out of stock' }}
                </span>
            </div>
            <div
                class="py-2 px-3 w-full text-sm bg-navy rounded-full text-white text-center mt-3 flex items-center justify-center">
                <img class="mr-2" src="https://api.iconify.design/mdi:eye.svg?color=%23ffffff" width="20px">
                <strong>View product</strong>
            </div>
        </div>
    </NuxtLink>
</template>
<script setup lang="js">
const config = useRuntimeConfig().public;
defineProps({
    product: {
        type: Object,
        default: () => []
    }
});
</script>