<template>
    <section class="w-full min-h-[80vh] px-4 py-10">
        <div v-if="processing" class="max-w-5xl mx-auto">
            <p class="text-zinc-500">Loading product...</p>
        </div>

        <div v-else-if="!product" class="max-w-5xl mx-auto text-center py-20">
            <h1 class="text-xl font-bold text-zinc-900">Product not found</h1>
            <p class="text-sm text-zinc-500 mt-2">The product you're looking for doesn't exist or is no longer
                available.</p>
            <NuxtLink to="/" class="inline-block mt-4 text-lime-main hover:underline">Back to home</NuxtLink>
        </div>

        <div class="w-full max-w-5xl mx-auto" v-else>
            <div class="max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8">
                <div class="w-full aspect-square rounded-lg overflow-hidden bgfader flex items-center justify-center">
                    <img v-if="product.image_url" :src="product.image_url" :alt="product.name"
                        class="w-full h-full object-contain" />
                    <span v-else class="text-zinc-400 text-sm">No image available</span>
                </div>

                <div class="flex flex-col gap-4">
                    <div>
                        <div v-if="product.brand?.images.length > 0" class="text-sm text-zinc-500 mb-3">
                            <img :src="product.brand.images[0].file_path" width="90px">
                        </div>
                        <p v-if="product.brand" class="text-sm text-zinc-500">{{ product.brand.name }}</p>
                        <h1 class="text-2xl font-bold text-zinc-900">{{ product.name }}</h1>
                        <p v-if="product.model" class="text-sm text-zinc-500">Model: {{ product.model }}</p>
                    </div>

                    <div class="flex items-baseline gap-3">
                        <span class="text-2xl font-bold text-navy">Rs. {{ formatPrice(product.selling_price) }}</span>
                    </div>

                    <p v-if="stockLabel" :class="stockClass" class="text-sm font-semibold">{{ stockLabel }}</p>

                    <p v-if="product.description" class="text-zinc-700 leading-relaxed">{{ product.description }}</p>

                    <div v-if="specs.length" class="mt-2">
                        <h2 class="text-sm font-semibold text-zinc-700 mb-2">Specifications</h2>
                        <dl class="grid grid-cols-2 gap-y-2 text-sm">
                            <template v-for="spec in specs" :key="spec.label">
                                <dt class="text-zinc-500">{{ spec.label }}</dt>
                                <dd class="text-zinc-700">{{ spec.value }}</dd>
                            </template>
                        </dl>
                    </div>

                    <p v-if="product.warranty_months" class="text-sm text-zinc-500">
                        {{ product.warranty_months }} month warranty
                    </p>

                    <div class="flex items-center gap-3" v-if="product.in_stock">
                        <button type="button" :disabled="!product.in_stock" @click="addToCart"
                            class="mt-4 px-4 py-2 rounded-md text-sm font-semibold border border-navy bg-navy text-white hover:bg-lime-hover hover:text-black hover:border-lime-hover transition-colors w-fit disabled:opacity-40 disabled:cursor-not-allowed">
                            {{ product.in_stock ? 'Add to Cart' : 'Out of Stock' }}
                        </button>
                        <button type="button" :disabled="!product.in_stock" @click="addToCart"
                            class="mt-4 px-4 py-2 rounded-md text-sm font-semibold border border-navy bg-transparent text-navy hover:bg-lime-hover hover:border-lime-hover hover:text-black transition-colors w-fit disabled:opacity-40 disabled:cursor-not-allowed">
                            {{ product.in_stock ? 'Buy now' : 'Out of Stock' }}
                        </button>
                    </div>
                </div>
            </div>
            <div class="mt-6 border-t border-gray-200 py-4" v-if="suggested_products.length > 0">
                <h1 class="text-2xl font-bold text-gray-800 py-2">Suggested products</h1>
                <AppProducts :products="suggested_products" />
            </div>
        </div>
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'product'
});

const { publicFetch } = usePublicFetch();

const product = ref(null);
const suggested_products = ref([]);
const processing = ref(true);

const route = useRoute();
const slug = route.params.slug;

try {
    const data = await publicFetch('/api/public/products/' + slug);
    if (data) {
        product.value = data;
        suggested_products.value = data.suggested_products;
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

const specs = computed(() => {
    if (!product.value) return [];
    const list = [];
    if (product.value.power_rating_watts) list.push({ label: 'Power', value: `${product.value.power_rating_watts} W` });
    if (product.value.voltage_rating) list.push({ label: 'Voltage', value: `${product.value.voltage_rating} V` });
    if (product.value.capacity_ah) list.push({ label: 'Capacity', value: `${product.value.capacity_ah} Ah` });
    return list;
});

const stockLabel = computed(() => {
    if (!product.value) return '';
    if (product.value.quantity_in_stock <= 0) return 'Out of stock';
    if (product.value.quantity_in_stock <= product.value.reorder_level) return 'Low stock';
    return 'In stock';
});

const stockClass = computed(() => {
    if (!product.value) return '';
    if (product.value.quantity_in_stock <= 0) return 'text-red-400';
    if (product.value.quantity_in_stock <= product.value.reorder_level) return 'text-yellow-600';
    return 'text-green-600';
});

function formatPrice(value) {
    const n = Number(value);
    if (Number.isNaN(n)) return value;
    return n.toLocaleString('en-PK', { minimumFractionDigits: 0, maximumFractionDigits: 2 });
}

useSeoMeta({
    title: () => product.value?.name || 'Product',
    description: () => product.value?.description || ''
});
</script>