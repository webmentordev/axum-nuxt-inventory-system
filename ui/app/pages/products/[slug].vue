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
                        <h1 class="text-2xl font-bold text-zinc-900 my-2">{{ product.name }}</h1>
                        <p v-if="product.model" class="text-sm text-zinc-500">Model: {{ product.model }}</p>
                    </div>

                    <div class="flex flex-col gap-1">
                        <div class="flex items-baseline gap-3">
                            <span class="text-2xl font-bold text-navy">Rs. {{ formatPrice(product.selling_price)
                            }}</span>
                            <span v-if="hasDiscount" class="text-base text-zinc-400 line-through">
                                Rs. {{ formatPrice(product.compare_at_selling_price) }}
                            </span>
                            <span v-if="hasDiscount"
                                class="text-xs font-semibold text-green-700 bg-green-100 px-2 py-0.5 rounded">
                                {{ discountPercent }}% off
                            </span>
                        </div>
                        <span v-if="isSolar && pricePerWatt" class="text-sm text-zinc-500">
                            Rs. {{ formatPrice(pricePerWatt) }} / watt
                        </span>
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

                    <div v-if="isSolar && solarSpecs.length" class="mt-2">
                        <h2 class="text-sm font-semibold text-zinc-700 mb-2">Solar Panel Specifications</h2>
                        <dl class="grid grid-cols-2 gap-y-2 text-sm">
                            <template v-for="spec in solarSpecs" :key="spec.label">
                                <dt class="text-zinc-500">{{ spec.label }}</dt>
                                <dd class="text-zinc-700">{{ spec.value }}</dd>
                            </template>
                        </dl>
                    </div>

                    <p v-if="product.warranty_months" class="text-sm text-zinc-500">
                        {{ product.warranty_months }} month warranty
                    </p>

                    <div v-if="product.content" class="mt-2">
                        <h2 class="text-sm font-semibold text-zinc-700 mb-2">Product Details</h2>
                        <p class="text-zinc-700 leading-relaxed whitespace-pre-line">{{ product.content }}</p>
                    </div>

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

const isSolar = computed(() => product.value?.product_type === 'solar');

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

const pricePerWatt = computed(() => {
    if (!product.value) return null;
    const watts = Number(product.value.power_rating_watts);
    const selling = Number(product.value.selling_price);
    if (!watts || Number.isNaN(watts) || Number.isNaN(selling)) return null;
    return selling / watts;
});

const specs = computed(() => {
    if (!product.value) return [];
    const list = [];
    if (product.value.power_rating_watts) list.push({ label: 'Power', value: `${product.value.power_rating_watts} W` });
    if (product.value.voltage_rating) list.push({ label: 'Voltage', value: `${product.value.voltage_rating} V` });
    if (product.value.capacity_ah) list.push({ label: 'Capacity', value: `${product.value.capacity_ah} Ah` });
    return list;
});

const solarSpecs = computed(() => {
    if (!product.value) return [];
    const p = product.value;
    const list = [];
    if (p.panel_type) list.push({ label: 'Panel Type', value: p.panel_type });
    if (p.cell_type) list.push({ label: 'Cell Type', value: p.cell_type });
    if (p.number_of_cells) list.push({ label: 'Number of Cells', value: p.number_of_cells });
    if (p.efficiency_percentage) list.push({ label: 'Efficiency', value: `${p.efficiency_percentage}%` });
    if (p.max_system_voltage) list.push({ label: 'Max System Voltage', value: `${p.max_system_voltage} V` });
    if (p.open_circuit_voltage) list.push({ label: 'Open Circuit Voltage', value: `${p.open_circuit_voltage} V` });
    if (p.short_circuit_current) list.push({ label: 'Short Circuit Current', value: `${p.short_circuit_current} A` });
    if (p.max_power_voltage) list.push({ label: 'Max Power Voltage', value: `${p.max_power_voltage} V` });
    if (p.max_power_current) list.push({ label: 'Max Power Current', value: `${p.max_power_current} A` });
    if (p.temperature_coefficient) list.push({ label: 'Temp. Coefficient', value: `${p.temperature_coefficient}%/°C` });
    if (p.frame_material) list.push({ label: 'Frame Material', value: p.frame_material });
    if (p.glass_type) list.push({ label: 'Glass Type', value: p.glass_type });
    if (p.length_mm) list.push({ label: 'Length', value: `${p.length_mm} mm` });
    if (p.width_mm) list.push({ label: 'Width', value: `${p.width_mm} mm` });
    if (p.thickness_mm) list.push({ label: 'Thickness', value: `${p.thickness_mm} mm` });
    if (p.weight_kg) list.push({ label: 'Weight', value: `${p.weight_kg} kg` });
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