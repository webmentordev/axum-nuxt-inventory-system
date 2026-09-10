<template>
    <NuxtLink :to='`/products/${product.slug}`' class="overflow-hidden flex flex-col relative bgfader-hover"
        :title="product.name">
        <NuxtImg :src="product.image_url" width="400" height="160" :alt='`${product.name} Image`'
            class="w-full h-28 sm:h-32 lg:h-40 object-contain" loading="lazy" />
        <img v-if="product.brand?.uploads.length" :src="product.brand?.uploads[0].file_path"
            class="absolute top-1 left-1 w-8 sm:w-[50px]" loading="lazy" />
        <span v-if="hasDiscount"
            class="absolute top-1 right-1 text-[10px] sm:text-xs font-semibold px-2 py-1 rounded-full bg-green-100 text-green-700">
            {{ discountPercent }}% off
        </span>
        <span v-else class="text-[10px] sm:text-xs px-2 py-1 rounded-full absolute top-1 right-1" :class="product.in_stock
            ? 'bg-green-100 text-green-700'
            : 'bg-red-100 text-red-700'">
            {{ product.in_stock ? 'In stock' : 'Out of stock' }}
        </span>
        <div class="p-2 sm:p-3 flex flex-col flex-1">
            <h2 class="font-semibold text-xs sm:text-sm line-clamp-2">{{ product.name }}</h2>
            <p class="text-xs text-gray-500 mt-1">
                {{ product.brand ? product.brand.name : 'No brand' }}
                <span v-if="product.model"> · {{ product.model }}</span>
            </p>
            <div class="mt-auto pt-3 flex items-center justify-between">
                <div class="flex flex-col">
                    <div class="font-bold text-sm sm:text-lg flex items-baseline gap-1.5">
                        {{ formatCurrency(Number(product.selling_price)) }}
                    </div>
                    <span v-if="hasDiscount" class="text-xs text-gray-400 line-through">
                        {{ formatCurrency(Number(product.compare_at_selling_price)) }}
                    </span>
                </div>
                <span v-if="hasDiscount" class="text-[10px] sm:text-xs px-2 py-1 rounded-full" :class="product.in_stock
                    ? 'bg-green-100 text-green-700'
                    : 'bg-red-100 text-red-700'">
                    {{ product.in_stock ? 'In stock' : 'Out of stock' }}
                </span>
            </div>
            <div
                class="py-2 px-3 w-full text-xs sm:text-sm bg-navy rounded-md text-white text-center mt-3 flex items-center justify-center">
                <strong class="mr-2 uppercase">View product</strong>
                <Icon name="mdi:eye-outline" class="translate-y-0.5" size="18px" />
            </div>
        </div>
    </NuxtLink>
</template>
<script setup lang="js">
const props = defineProps({
    product: {
        type: Object,
        default: () => []
    }
});

const hasDiscount = computed(() => {
    const compareAt = Number(props.product?.compare_at_selling_price);
    const selling = Number(props.product?.selling_price);
    return !Number.isNaN(compareAt) && compareAt > selling;
});

const discountPercent = computed(() => {
    if (!hasDiscount.value) return 0;
    const compareAt = Number(props.product.compare_at_selling_price);
    const selling = Number(props.product.selling_price);
    return Math.round(((compareAt - selling) / compareAt) * 100);
});

function formatCurrency(amount) {
    const currency = useRuntimeConfig().public.currency;
    return new Intl.NumberFormat('en-PK', {
        style: 'currency',
        currency: currency
    }).format(Number(amount));
}
</script>