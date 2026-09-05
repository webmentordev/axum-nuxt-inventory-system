<template>
    <section class="w-full min-h-[80vh] px-4 py-10">
        <div v-if="processing" class="max-w-5xl mx-auto">
            <p class="text-zinc-500">Loading product...</p>
        </div>

        <div v-else-if="!product" class="max-w-5xl mx-auto text-center py-20">
            <h1 class="text-xl font-bold text-zinc-900">Product not found</h1>
            <p class="text-sm text-zinc-500 mt-2">The product you're looking for doesn't exist or is no longer
                available.</p>
            <NuxtLink to="/" class="inline-block mt-4 text-orange hover:underline">Back to home</NuxtLink>
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
                        <div v-if="brandLogo" class="text-sm text-zinc-500 mb-3">
                            <img :src="brandLogo" width="90px">
                        </div>
                        <h1 class="text-2xl font-bold text-zinc-900 my-2">{{ product.name }}</h1>
                        <p v-if="product.model" class="text-sm text-zinc-500">Model: {{ product.model }}</p>
                    </div>

                    <ul class="text-sm flex flex-col gap-1">
                        <li><strong>SKU: </strong>{{ product.sku }}</li>
                        <li v-if="product.brand"><strong>Brand: </strong>
                            <NuxtLink class="underline" :to='`/brands/${product.brand.slug}`' target="_blank">{{
                                product.brand.name }}
                            </NuxtLink>
                        </li>
                        <li v-if="product.category"><strong>Category: </strong>
                            <NuxtLink class="underline" :to='`/categories/${product.category.slug}`' target="_blank">{{
                                product.category.name }}
                            </NuxtLink>
                        </li>
                        <li v-if="product.sub_category"><strong>Tech Type: </strong>
                            <NuxtLink class="underline" :to='`/sub-categories/${product.sub_category.slug}`'
                                target="_blank">{{
                                    product.sub_category.name
                                }}</NuxtLink>
                        </li>
                        <li v-if="stockLabel"><strong>Availability: </strong><span :class="stockClass">{{ stockLabel
                                }}</span></li>
                    </ul>

                    <div class="flex flex-col gap-1">
                        <div class="flex items-baseline gap-3">
                            <span class="text-2xl font-bold text-navy">{{ formatCurrency(product.selling_price)
                                }}</span>
                            <span v-if="hasDiscount" class="text-base text-zinc-400 line-through">
                                {{ formatCurrency(product.compare_at_selling_price) }}
                            </span>
                            <span v-if="hasDiscount"
                                class="text-xs font-semibold text-green-700 bg-green-100 px-2 py-0.5 rounded">
                                {{ discountPercent }}% off
                            </span>
                        </div>
                        <span v-if="displayPricePerWatt" class="text-sm text-zinc-500">
                            {{ formatCurrency(displayPricePerWatt) }} / watt
                        </span>
                    </div>

                    <p v-if="product.description" class="text-zinc-700 leading-relaxed">{{ product.description }}</p>

                    <ul v-if="specs.length" class="text-sm text-zinc-600 flex flex-col gap-1">
                        <li v-for="spec in specs" :key="spec.label">
                            <strong>{{ spec.label }}: </strong>{{ spec.value }}
                        </li>
                    </ul>

                    <p v-if="product.warranty_months" class="text-sm text-zinc-500">
                        {{ product.warranty_months }} month warranty
                    </p>

                    <div class="flex items-center gap-3" v-if="product.in_stock">
                        <button type="button" :disabled="!product.in_stock" @click="addToCart"
                            class="mt-4 px-4 py-2 rounded-md text-sm font-semibold border border-navy bg-navy text-white hover:bg-orange hover:text-black hover:border-orange transition-colors w-fit disabled:opacity-40 disabled:cursor-not-allowed">
                            {{ product.in_stock ? 'Add to Cart' : 'Out of Stock' }}
                        </button>
                        <button type="button" :disabled="!product.in_stock" @click="addToCart"
                            class="mt-4 px-4 py-2 rounded-md text-sm font-semibold border border-navy bg-transparent text-navy hover:bg-orange hover:border-orange hover:text-black transition-colors w-fit disabled:opacity-40 disabled:cursor-not-allowed">
                            {{ product.in_stock ? 'Buy now' : 'Out of Stock' }}
                        </button>
                    </div>
                </div>
            </div>
            <div v-if="product.content">
                <h3 class="text-xl font-semibold mb-2" :title='`${product.name} details and specifications`'>Product
                    description
                </h3>
                <article class="product prose prose-neutral max-w-none" v-html="product.content">
                </article>
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
const seo = ref(null);
const suggested_products = ref([]);
const processing = ref(true);

const route = useRoute();
const slug = route.params.slug;

const siteUrl = useRuntimeConfig().public?.siteUrl || '';
const canonicalUrl = computed(() => `${siteUrl}/policies/${slug}`);

const { addToCart: addProductToCart } = useCart();

function addToCart() {
    addProductToCart(product.value, 1);
}

try {
    const data = await publicFetch('/api/public/products/' + slug);
    if (data) {
        product.value = data;
        suggested_products.value = data.suggested_products;
        seo.value = data.seo;
        if (seo.value) {
            useSeoMeta({
                title: () => product.value?.name || 'Product',
                description: () => seo.value?.meta_description || undefined,
                keywords: () => seo.value?.meta_keywords || undefined,
                ogTitle: () => seo.value?.og_title || seo.value?.meta_title || product.value?.name || 'Product',
                ogDescription: () => seo.value?.og_description || seo.value?.meta_description || undefined,
                ogImage: () => seo.value?.og_image_url || undefined,
                ogUrl: () => canonicalUrl.value,
                ogType: 'article',
                twitterCard: 'summary_large_image',
                twitterTitle: () => seo.value?.og_title || seo.value?.meta_title || product.value?.name || 'Product',
                twitterDescription: () => seo.value?.og_description || seo.value?.meta_description || undefined
            });

            useHead({
                meta: [
                    { property: 'product:price:amount', content: () => product.value?.selling_price },
                    { property: 'product:price:currency', content: 'PKR' },
                    { property: 'og:availability', content: () => product.value?.in_stock ? 'instock' : 'oos' }
                ],
                link: [
                    {
                        rel: 'canonical',
                        href: seo.value?.canonical_url || canonicalUrl.value
                    }
                ]
            });
        }
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

const brandLogo = computed(() => {
    const brandUploads = product.value?.brand?.uploads || [];
    const logo = brandUploads.find((u) => u.file_type === 'image');
    return logo?.file_path || null;
});

const hasDiscount = computed(() => {
    if (!product.value) return false;
    const compareAt = Number(product.value.compare_at_selling_price);
    const selling = Number(product.value.selling_price);
    return !Number.isNaN(compareAt) && compareAt > selling;
});

const discountPercent = computed(() => {
    if (!hasDiscount.value) return 0;
    const compareAt = Number(product.value.compare_at_selling_price);
    const selling = Number(product.value.selling_price);
    return Math.round(((compareAt - selling) / compareAt) * 100);
});

const displayPricePerWatt = computed(() => {
    if (!product.value) return null;

    const perWatt = Number(product.value.per_watt_price);
    return Number.isNaN(perWatt) || !product.value.per_watt_price ? null : perWatt;
});

const specs = computed(() => {
    if (!product.value) return [];
    const p = product.value;
    const list = [];
    if (p.power_rating_watts) list.push({ label: 'Power', value: `${p.power_rating_watts} W` });
    if (p.voltage_rating) list.push({ label: 'Voltage', value: `${p.voltage_rating} V` });
    if (p.capacity_ah) list.push({ label: 'Capacity', value: `${p.capacity_ah} Ah` });
    if (p.kilowatt_hour) list.push({ label: 'Kilowatt Hour', value: `${p.kilowatt_hour} kWh` });
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
    if (product.value.quantity_in_stock <= 0) return 'text-red-400 font-semibold';
    if (product.value.quantity_in_stock <= product.value.reorder_level) return 'text-yellow-600 font-semibold';
    return 'text-green-600 font-semibold';
});

function formatCurrency(amount) {
    const currency = useRuntimeConfig().public.currency;
    return new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: currency
    }).format(Number(amount));
}

</script>

<style scoped>
.product :deep(a) {
    color: #2563eb;
    text-decoration: underline;
}

.product :deep(h2) {
    margin-top: 1.5rem;
    margin-bottom: 0.75rem;
    font-size: 1.25rem;
    font-weight: 600;
}

.product :deep(p) {
    margin-bottom: 1rem;
    line-height: 1.7;
}

.product :deep(ul),
.product :deep(ol) {
    margin-bottom: 1rem;
    padding-left: 1.5rem;
}

.product :deep(li) {
    margin-bottom: 0.25rem;
}
</style>