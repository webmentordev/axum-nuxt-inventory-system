<template>
    <section class="relative w-full max-w-3xl m-auto" ref="searchRef">
        <div class="flex items-center">
            <input type="search" v-model="search" @keyup.enter="runSearch" @focus="showDropdown = true"
                placeholder="Search by name, brand..."
                class="bg-slate-200 rounded-full w-full py-2 outline-none px-5 text-sm">
            <button @click="runSearch"
                class="bg-navy shrink-0 flex items-center text-white py-2 px-4 rounded-full ml-2">
                <img src="https://api.iconify.design/ic:outline-search.svg?color=%23ffffff" width="18px">
                <strong class="text-sm ml-1 -translate-y-0.5">Search</strong>
            </button>
        </div>

        <div v-if="showDropdown && search.trim()"
            class="absolute left-0 right-0 mt-2 rounded-md border border-gray-200 bg-white shadow-lg text-sm z-50 overflow-hidden">

            <div v-if="loading" class="px-4 py-6 text-center text-zinc-500">
                Searching...
            </div>

            <div v-else-if="error" class="px-4 py-6 text-center text-red-500">
                {{ error }}
            </div>

            <div v-else-if="products.length === 0" class="px-4 py-6 text-center text-zinc-500">
                No products found for "{{ search }}"
            </div>

            <div v-else class="flex flex-col">
                <div class="max-h-96 overflow-y-auto flex flex-col divide-y divide-gray-100">
                    <NuxtLink v-for="product in products" :key="product.id" :to="`/products/${product.slug}`"
                        @click="closeDropdown" class="flex items-center gap-3 px-4 py-2 hover:bg-gray-50">
                        <div
                            class="w-12 h-12 rounded-md overflow-hidden bgfader flex items-center justify-center shrink-0">
                            <img v-if="product.image_url" :src="product.image_url" :alt="product.name"
                                class="w-full h-full object-contain" />
                            <span v-else class="text-zinc-400 text-[10px]">No image</span>
                        </div>

                        <div class="flex-1 min-w-0">
                            <p class="text-xs font-semibold text-zinc-900 truncate">{{ product.name }}</p>
                            <p class="text-[11px] text-zinc-500 truncate">
                                {{ product.brand?.name }}<span v-if="product.brand">&nbsp;·&nbsp;</span>{{ product.sku
                                }}
                            </p>
                        </div>

                        <div class="text-right shrink-0">
                            <p class="text-xs font-semibold text-navy">Rs. {{ formatCurrency(product.selling_price) }}
                            </p>
                            <p class="text-[10px]" :class="product.in_stock ? 'text-green-600' : 'text-red-500'">
                                {{ product.in_stock ? 'In stock' : 'Out of stock' }}
                            </p>
                        </div>
                    </NuxtLink>
                </div>

                <NuxtLink :to="`/search?q=${encodeURIComponent(search)}`" @click="closeDropdown"
                    class="block px-4 py-2 text-center text-xs font-semibold text-navy border-t border-gray-100 hover:bg-gray-50">
                    View all results ({{ total }})
                </NuxtLink>
            </div>
        </div>
    </section>
</template>

<script setup lang="js">
const { publicFetch } = usePublicFetch();

const search = ref("");
const products = ref([]);
const total = ref(0);
const loading = ref(false);
const error = ref("");
const showDropdown = ref(false);
const searchRef = ref(null);

let debounceTimer = null;
let requestId = 0;

async function runSearch() {
    const query = search.value.trim();
    showDropdown.value = true;

    if (!query) {
        products.value = [];
        total.value = 0;
        error.value = "";
        return;
    }

    const currentRequest = ++requestId;
    loading.value = true;
    error.value = "";

    try {
        const res = await publicFetch("/api/public/products/search", {
            method: "POST",
            body: {
                query,
                limit: 8,
                offset: 0
            }
        });

        if (currentRequest !== requestId) return;

        products.value = res.products;
        total.value = res.total;
    } catch (e) {
        if (currentRequest !== requestId) return;
        error.value = "Something went wrong. Try again.";
        products.value = [];
    } finally {
        if (currentRequest === requestId) loading.value = false;
    }
}

watch(search, () => {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(runSearch, 300);
});

function closeDropdown() {
    showDropdown.value = false;
}

function handleClickOutside(e) {
    if (searchRef.value && !searchRef.value.contains(e.target)) {
        showDropdown.value = false;
    }
}

onMounted(() => document.addEventListener("click", handleClickOutside));
onBeforeUnmount(() => document.removeEventListener("click", handleClickOutside));

function formatCurrency(value) {
    return Number(value).toLocaleString("en-PK", { minimumFractionDigits: 0 });
}
</script>