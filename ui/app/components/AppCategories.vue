<template>
    <section class="w-full">
        <div class="max-w-2xl m-auto p-3">
            <Loading v-if="processing" message="Loading categories..." />
            <div v-else-if="categories.length">
                <div class="flex justify-between items-center mb-3 pb-3 border-b border-gray-300">
                    <h3 class="font-bold uppercase">Categories</h3>
                    <Icon name="basil:caret-down-solid" size="30px" />
                </div>
                <div class="flex flex-col">
                    <div v-for="category in categories" :key="category.slug">
                        <div class="flex justify-between items-center py-1">
                            <NuxtLink :to="`/categories/${category.slug}`">{{ category.name }}</NuxtLink>

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
                                :to="`/sub-categories/${sub.slug}`" class="py-1 text-sm text-gray-600">
                                {{ sub.name }}
                            </NuxtLink>
                        </div>
                    </div>
                </div>
            </div>
            <p v-if="!processing && categories.length == 0">No category exist.</p>

            <Loading v-if="brandsProcessing" message="Loading brands..." />
            <div v-else-if="brands.length" class="mt-4">
                <div class="flex justify-between items-center mb-3 pb-3 border-b border-gray-300">
                    <h3 class="font-bold uppercase">Brands</h3>
                </div>
                <div class="flex flex-col">
                    <div v-for="brand in brands" :key="brand.slug" class="py-1">
                        <NuxtLink :to="`/brands/${brand.slug}`">{{ brand.name }}</NuxtLink>
                    </div>
                </div>
            </div>
            <p v-if="!brandsProcessing && brands.length == 0">No brand exist.</p>
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