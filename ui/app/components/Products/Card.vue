<template>
    <NuxtLink :to='`/products/${product.slug}`' class="overflow-hidden flex flex-col relative bgfader-hover"
        :title="product.name">
        <NuxtImg :src="product.image_url" width="400" height="160" :alt='`${product.name} Image`'
            class="w-full h-40 object-contain" loading="lazy" />
        <img v-if="product.brand?.uploads.length" :src="product.brand?.uploads[0].file_path"
            class="absolute top-1 left-1" loading="lazy" width="50px" />
        <div class="p-3 flex flex-col flex-1">
            <h2 class="font-semibold text-sm line-clamp-2">{{ product.name }}</h2>
            <p class="text-xs text-gray-500 mt-1">
                {{ product.brand ? product.brand.name : 'No brand' }}
                <span v-if="product.model"> · {{ product.model }}</span>
            </p>
            <div class="mt-auto pt-3 flex items-center justify-between">
                <div class="font-bold text-lg">
                    {{ formatCurrency(Number(product.selling_price)) }}
                    <span class="text-sm text-gray-500" v-if="product.product_type == 'solar'">/{{
                        product.per_watt_price }} W</span>
                </div>
                <span class="text-xs px-2 py-1 rounded-full absolute top-1 right-1" :class="product.in_stock
                    ? 'bg-green-100 text-green-700'
                    : 'bg-red-100 text-red-700'">
                    {{ product.in_stock ? 'In stock' : 'Out of stock' }}
                </span>
            </div>
            <div
                class="py-2 px-3 w-full text-sm bg-navy rounded-md text-white text-center mt-3 flex items-center justify-center">
                <strong class="mr-2 uppercase">View product</strong>
                <Icon name="mdi:eye-outline" class="translate-y-0.5" size="20px" />
            </div>
        </div>
    </NuxtLink>
</template>
<script setup lang="js">
defineProps({
    product: {
        type: Object,
        default: () => []
    }
});

function formatCurrency(amount) {
    const currency = useRuntimeConfig().public.currency;
    return new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: currency
    }).format(Number(amount));
}
</script>