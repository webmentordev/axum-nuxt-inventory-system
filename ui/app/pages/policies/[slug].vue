<template>
    <section class="w-full min-h-[80vh] py-10">
        <div class="max-w-2xl mx-auto px-4">
            <div v-if="processing" class="animate-pulse space-y-4">
                <div class="h-8 bg-gray-200 rounded w-1/2"></div>
                <div class="h-4 bg-gray-200 rounded w-1/3"></div>
                <div class="h-4 bg-gray-200 rounded w-full"></div>
                <div class="h-4 bg-gray-200 rounded w-full"></div>
                <div class="h-4 bg-gray-200 rounded w-2/3"></div>
            </div>

            <div v-else-if="policy">
                <header class="mb-6 border-b pb-4">
                    <h1 class="text-3xl font-bold text-gray-900">{{ policy.name }}</h1>
                    <p class="text-sm text-gray-500 mt-1">
                        Last updated: {{ formatDate(policy.updated_at) }}
                    </p>
                </header>

                <article class="policy prose prose-neutral max-w-none" v-html="policy.content"></article>
            </div>

            <div v-else class="text-center text-gray-500 py-20">
                Policy not found.
            </div>
        </div>
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'guest'
});

const { publicFetch } = usePublicFetch();

const route = useRoute();
const runtimeConfig = useRuntimeConfig();
const slug = route.params.slug;

const policy = ref(null);
const processing = ref(true);

function formatDate(utcString) {
    if (!utcString) return '';
    return new Date(utcString).toLocaleString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: 'numeric',
        minute: '2-digit'
    });
}

try {
    const data = await publicFetch('/api/public/policies/' + slug);
    if (data) {
        policy.value = data;
    }
} catch (e) {
    throw createError({
        statusCode: e.statusCode || 500,
        statusMessage: e.statusMessage || 'Something went wrong!',
        fatal: true
    });
} finally {
    processing.value = false;
}

const siteUrl = useRuntimeConfig().public?.siteUrl || '';
const canonicalUrl = computed(() => `${siteUrl}/policies/${slug}`);

useSeoMeta({
    title: () => policy.value?.seo_title || policy.value?.name || 'Policy',
    description: () => policy.value?.seo_description || undefined,
    ogTitle: () => policy.value?.seo_title || policy.value?.name || 'Policy',
    ogDescription: () => policy.value?.seo_description || undefined,
    ogUrl: () => canonicalUrl.value,
    ogType: 'article',
    twitterCard: 'summary_large_image',
    twitterTitle: () => policy.value?.seo_title || policy.value?.name || 'Policy',
    twitterDescription: () => policy.value?.seo_description || undefined
});

useHead({
    link: [
        {
            rel: 'canonical',
            href: canonicalUrl.value
        }
    ]
});
</script>

<style scoped>
.policy :deep(a) {
    color: #2563eb;
    text-decoration: underline;
}

.policy :deep(h2) {
    margin-top: 1.5rem;
    margin-bottom: 0.75rem;
    font-size: 1.25rem;
    font-weight: 600;
}

.policy :deep(p) {
    margin-bottom: 1rem;
    line-height: 1.7;
}

.policy :deep(ul),
.policy :deep(ol) {
    margin-bottom: 1rem;
    padding-left: 1.5rem;
}

.policy :deep(li) {
    margin-bottom: 0.25rem;
}
</style>