<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-2xl font-bold text-white">{{ productTitle }}</h1>
            <h2 class="text-lg font-bold text-zinc-400">Update Product SEO</h2>
            <p class="text-sm text-zinc-500 mt-1">Update SEO metadata for this product.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Meta Title</label>
                    <AdminInput v-model="metaTitle" placeholder="e.g. Buy Jinko Solar Panels Online" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Meta Description</label>
                    <AdminInput v-model="metaDescription" type="textarea"
                        placeholder="Short description for search engines" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Meta Keywords</label>
                    <AdminInput v-model="metaKeywords" placeholder="e.g. solar panel, jinko, renewable energy" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">OG Title</label>
                    <AdminInput v-model="ogTitle" placeholder="e.g. Jinko Solar Panels" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">OG Description</label>
                    <AdminInput v-model="ogDescription" type="textarea"
                        placeholder="Description shown when shared on social media" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">OG Image URL</label>
                    <AdminInput v-model="ogImageUrl" placeholder="https://..." />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Canonical URL (No https or
                        domain)</label>
                    <AdminInput v-model="canonicalUrl" placeholder="https://..." />
                    <p class="text-lime-300 text-sm mt-1">{{ productUrl }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Focus Keyword</label>
                    <AdminInput v-model="focusKeyword" placeholder="e.g. solar panel" />
                </div>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Update SEO
                </button>
            </form>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();
const route = useRoute();
const seoId = route.params.id;

const metaTitle = ref('');
const metaDescription = ref('');
const metaKeywords = ref('');
const ogTitle = ref('');
const ogDescription = ref('');
const ogImageUrl = ref('');
const canonicalUrl = ref('');
const focusKeyword = ref('');
const productTitle = ref('');
const productUrl = ref('');

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

async function loadSeo() {
    try {
        const data = await authFetch(`/api/admin/seo/${seoId}`);
        if (data) {
            metaTitle.value = data.meta_title || '';
            metaDescription.value = data.meta_description || '';
            metaKeywords.value = data.meta_keywords || '';
            ogTitle.value = data.og_title || '';
            ogDescription.value = data.og_description || '';
            ogImageUrl.value = data.og_image_url || '';
            canonicalUrl.value = data.canonical_url || '';
            focusKeyword.value = data.focus_keyword || '';
            productTitle.value = data.product.name || '';
            productUrl.value = "/products/" + data.product.slug || '';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to load SEO.';
        showStatus.value = true;
    }
}

async function handleSubmit() {
    statusType.value = 'loading';
    statusMessage.value = 'Updating SEO...';
    showStatus.value = true;

    try {
        const data = await authFetch(`/api/admin/seo/${seoId}`, {
            method: 'PATCH',
            body: {
                meta_title: metaTitle.value.trim() || null,
                meta_description: metaDescription.value.trim() || null,
                meta_keywords: metaKeywords.value.trim() || null,
                og_title: ogTitle.value.trim() || null,
                og_description: ogDescription.value.trim() || null,
                og_image_url: ogImageUrl.value.trim() || null,
                canonical_url: canonicalUrl.value.trim() || null,
                focus_keyword: focusKeyword.value.trim() || null
            }
        });

        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'SEO updated.';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update SEO.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

onMounted(() => {
    loadSeo();
});
</script>