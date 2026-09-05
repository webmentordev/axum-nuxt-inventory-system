<template>
    <section>
        <header class="min-h-180 bg-cover bg-center" style="background-image: url('/kaleem-solar-banner.png');">
        </header>
        <div class="max-w-7xl m-auto py-6" v-if="categories.length > 0">
            <h3 class="uppercase text-center font-semibold text-2xl text-navy">Shop by category</h3>
            <div class="h-0.5 max-w-25 bg-orange m-auto mt-2"></div>
            <div class="grid grid-cols-5 gap-4 mt-3">
                <div v-for="(category, index) in categories" :key="index">
                    <NuxtLink :to='`/categories/${category.slug}`'
                        class="overflow-hidden flex flex-col relative bgfader-hover" :title="category.name">
                        <NuxtImg v-if="category.uploads?.length > 0" :src="category.uploads[0].file_path" width="400"
                            height="160" :alt='`${category.uploads[0].file_path} Image`'
                            class="w-full h-40 object-contain" loading="lazy" />
                        <div class="p-3 flex flex-col flex-1">
                            <h2 class="font-semibold text-sm line-clamp-2 text-navy text-center m-auto mb-3 capitalize">
                                {{ category.name }}</h2>
                            <div class="flex items-center m-auto">
                                <span class="mr-2 text-navy text-sm capitalize">View products</span>
                                <Icon name="ic:outline-arrow-right-alt" class="text-orange" size="25px" />
                            </div>
                        </div>
                    </NuxtLink>
                </div>
            </div>
        </div>
    </section>
</template>

<script setup lang="js">
definePageMeta({
    layout: 'home'
});

const { categories, processing, fetchCategories } = useCategories({
    featured: true,
    withSubCategories: false,
    withUploads: true,
});

await fetchCategories();
</script>