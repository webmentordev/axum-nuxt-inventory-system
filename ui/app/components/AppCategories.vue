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
        </div>
    </section>
</template>

<script setup lang="js">
const { publicFetch } = usePublicFetch();

const categories = ref([]);
const processing = ref(true);
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

try {
    const data = await publicFetch('/api/public/categories?sub_categories=true&is_featured=true');
    if (data) {
        categories.value = data;
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
</script>