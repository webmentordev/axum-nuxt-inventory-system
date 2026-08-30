<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Create Product SEO</h1>
            <p class="text-sm text-zinc-500 mt-1">Attach SEO metadata to a product.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Product</label>
                    <AdminSelect v-model="productId" :options="productOptions"
                        :placeholder="productLoading ? 'Loading products...' : 'Select a product'" />
                    <p v-if="errors.product_id" class="text-xs text-red-400 mt-1">{{ errors.product_id }}</p>
                </div>

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
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Focus Keyword</label>
                    <AdminInput v-model="focusKeyword" placeholder="e.g. solar panel" />
                </div>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Create SEO
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

const productId = ref(null);
const productOptions = ref([]);
const productList = ref([]);
const productLoading = ref(false);

const metaTitle = ref('');
const metaDescription = ref('');
const metaKeywords = ref('');
const ogTitle = ref('');
const ogDescription = ref('');
const ogImageUrl = ref('');
const canonicalUrl = ref('');
const focusKeyword = ref('');

const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

watch(productId, (newId) => {
    if (!newId) return;
    const product = productList.value.find((p) => p.id === newId);
    if (!product) return;

    metaTitle.value = product.name;
    canonicalUrl.value = `/products/${product.slug}`;
});

async function loadProducts() {
    productLoading.value = true;
    try {
        const data = await authFetch('/api/admin/products/list');
        if (data) {
            productList.value = data;
            productOptions.value = data.map((item) => ({
                label: item.name,
                value: item.id
            }));
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Failed to load products.';
    } finally {
        productLoading.value = false;
    }
}

function validate() {
    errors.value = {};
    if (!productId.value) {
        errors.value.product_id = 'Please select a product.';
    }
    return Object.keys(errors.value).length === 0;
}

async function handleSubmit() {
    if (!validate()) return;

    statusType.value = 'loading';
    statusMessage.value = 'Creating SEO...';
    showStatus.value = true;

    try {
        const data = await authFetch('/api/admin/seo', {
            method: 'POST',
            body: {
                product_id: productId.value,
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
            statusMessage.value = 'SEO created.';
            productId.value = null;
            metaTitle.value = '';
            metaDescription.value = '';
            metaKeywords.value = '';
            ogTitle.value = '';
            ogDescription.value = '';
            ogImageUrl.value = '';
            canonicalUrl.value = '';
            focusKeyword.value = '';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to create SEO.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

onMounted(() => {
    loadProducts();
});
</script>