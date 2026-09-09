<template>
    <section class="w-full">
        <button type="button"
            class="flex lg:hidden items-center gap-2 w-full justify-between p-3 border border-gray-200 rounded-md"
            @click="drawerOpen = true">
            <span class="flex items-center gap-2 font-bold uppercase text-sm">
                <Icon name="basil:filter-solid" size="18px" />
                Browse categories
            </span>
            <Icon name="basil:caret-down-solid" size="18px" class="-rotate-90" />
        </button>

        <div v-if="drawerOpen" class="fixed inset-0 bg-black/50 z-40 lg:hidden" @click="drawerOpen = false"></div>

        <div class="fixed inset-y-0 left-0 z-50 w-72 max-w-[85%] bg-white shadow-xl overflow-y-auto transform transition-transform duration-300 p-4
                    lg:static lg:z-auto lg:w-full lg:max-w-2xl lg:shadow-none lg:transform-none lg:p-3 lg:overflow-visible"
            :class="drawerOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'">

            <div class="flex lg:hidden items-center justify-between mb-4 pb-3 border-b border-gray-200">
                <h3 class="font-bold uppercase">Browse</h3>
                <button type="button" @click="drawerOpen = false">
                    <Icon name="ic:round-close" size="24px" />
                </button>
            </div>

            <Loading v-if="processing" message="Loading categories..." />
            <div v-else-if="categories.length">
                <div class="flex justify-between items-center mb-3 pb-3 border-b border-gray-300">
                    <h3 class="font-bold uppercase text-sm sm:text-base">Categories</h3>
                </div>
                <div class="flex flex-col">
                    <div v-for="category in categories" :key="category.slug">
                        <div class="flex justify-between items-center py-1.5">
                            <NuxtLink :to="`/categories/${category.slug}`" class="text-sm sm:text-base"
                                @click="drawerOpen = false">{{ category.name }}</NuxtLink>

                            <button v-if="category.sub_categories?.length" type="button" class="p-1"
                                :aria-expanded="isExpanded(category.slug)" @click="toggleExpand(category.slug)">
                                <Icon name="basil:caret-down-solid" size="18px"
                                    class="transition-transform duration-200"
                                    :class="{ 'rotate-180': isExpanded(category.slug) }" />
                            </button>
                        </div>

                        <div v-if="category.sub_categories?.length && isExpanded(category.slug)"
                            class="flex flex-col pl-4 border-l border-gray-200">
                            <NuxtLink v-for="sub in category.sub_categories" :key="sub.slug"
                                :to="`/sub-categories/${sub.slug}`" class="py-1.5 text-xs sm:text-sm text-gray-600"
                                @click="drawerOpen = false">
                                {{ sub.name }}
                            </NuxtLink>
                        </div>
                    </div>
                </div>
            </div>
            <p v-if="!processing && categories.length == 0" class="text-sm text-gray-500">No category exist.</p>

            <Loading v-if="brandsProcessing" message="Loading brands..." />
            <div v-else-if="brands.length" class="mt-4">
                <div class="flex justify-between items-center mb-3 pb-3 border-b border-gray-300">
                    <h3 class="font-bold uppercase text-sm sm:text-base">Brands</h3>
                </div>
                <div class="flex flex-col">
                    <div v-for="brand in brands" :key="brand.slug" class="py-1.5">
                        <NuxtLink :to="`/brands/${brand.slug}`" class="text-sm sm:text-base"
                            @click="drawerOpen = false">{{ brand.name }}
                        </NuxtLink>
                    </div>
                </div>
            </div>
            <p v-if="!brandsProcessing && brands.length == 0" class="text-sm text-gray-500">No brand exist.</p>
        </div>
    </section>
</template>

<script setup lang="js">
const { publicFetch } = usePublicFetch();

const { data: categories, pending: processing } = await useAsyncData('categories-with-sub', () => {
    const params = new URLSearchParams({
        sub_categories: 'true',
        is_featured: 'false',
        with_uploads: 'false',
    });
    return publicFetch(`/api/public/categories?${params}`);
}, { default: () => [] });

const expandedCategories = ref(new Set());
const drawerOpen = ref(false);

const toggleExpand = (slug) => {
    if (expandedCategories.value.has(slug)) {
        expandedCategories.value.delete(slug);
    } else {
        expandedCategories.value.add(slug);
    }
    expandedCategories.value = new Set(expandedCategories.value);
};

const isExpanded = (slug) => expandedCategories.value.has(slug);

const { data: brands, pending: brandsProcessing } = await useAsyncData('brands', () =>
    publicFetch('/api/public/brands'),
    { default: () => [] }
);
</script>