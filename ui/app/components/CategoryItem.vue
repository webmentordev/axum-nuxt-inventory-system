<template>
    <NuxtLink :to='`/categories/${category.slug}`' class="overflow-hidden flex flex-col relative"
        :title="category.name">
        <NuxtImg v-if="category.uploads?.length > 0" :src="category.uploads[0].file_path" width="400" height="160"
            :alt='`${category.uploads[0].file_path} Image`' class="w-full h-40 object-contain" loading="lazy" />
        <div class="p-3 flex flex-col flex-1">
            <h2 class="font-semibold text-sm line-clamp-2 text-navy text-center m-auto mb-3 capitalize">
                {{ category.name }}</h2>
            <div class="flex items-center m-auto">
                <span class="mr-2 text-navy text-sm capitalize">View products</span>
                <Icon name="ic:outline-arrow-right-alt" class="text-orange" size="25px" />
            </div>
        </div>
    </NuxtLink>
</template>

<script setup lang="js">
defineProps({
    category: {
        type: Object,
        default: () => []
    }
});
</script>