<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Create policy</h1>
            <p class="text-sm text-zinc-500 mt-1">Add a new e-commerce policy.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Name</label>
                    <AdminInput v-model="name" placeholder="e.g. Refund Policy" />
                    <p v-if="errors.name" class="text-xs text-red-400 mt-1">{{ errors.name }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">SEO Title</label>
                    <AdminInput v-model="seoTitle" placeholder="e.g. Refund Policy | YourStore" />
                    <p v-if="errors.seo_title" class="text-xs text-red-400 mt-1">{{ errors.seo_title }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">SEO Description</label>
                    <AdminTextarea v-model="seoDescription" placeholder="Short description for search engines..."
                        rows="2" />
                    <p v-if="errors.seo_description" class="text-xs text-red-400 mt-1">{{ errors.seo_description }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Content</label>
                    <AdminTextarea v-model="content" placeholder="Policy content..." rows="10" />
                    <p v-if="errors.content" class="text-xs text-red-400 mt-1">{{ errors.content }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Sort Order</label>
                    <AdminInput v-model="sortOrder" type="number" step="1" placeholder="1" />
                    <p v-if="errors.sort_order" class="text-xs text-red-400 mt-1">{{ errors.sort_order }}</p>
                </div>

                <label class="flex items-center gap-2 w-fit cursor-pointer select-none">
                    <input type="checkbox" v-model="isActive"
                        class="h-4 w-4 rounded border-zinc-600 bg-zinc-800 text-lime-main focus:ring-lime-main" />
                    <span class="text-sm font-semibold text-zinc-300">Active</span>
                </label>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Create policy
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

const name = ref('');
const seoTitle = ref('');
const seoDescription = ref('');
const content = ref('');
const sortOrder = ref('1');
const isActive = ref(true);
const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

async function handleSubmit() {
    if (!validate()) return;
    statusType.value = 'loading';
    statusMessage.value = 'Creating policy...';
    showStatus.value = true;
    try {
        const data = await authFetch('/api/admin/policies', {
            method: 'POST',
            body: {
                name: name.value.trim(),
                seo_title: seoTitle.value.trim(),
                seo_description: seoDescription.value.trim(),
                content: content.value.trim(),
                sort_order: Number(sortOrder.value),
                is_active: isActive.value
            }
        });
        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Policy created.';
            name.value = '';
            seoTitle.value = '';
            seoDescription.value = '';
            content.value = '';
            sortOrder.value = '1';
            isActive.value = true;
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to create policy.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000)
    }
};

function validate() {
    errors.value = {};

    if (!name.value.trim()) {
        errors.value.name = 'Name is required.';
    } else if (name.value.trim().length < 2) {
        errors.value.name = 'Name must be at least 2 characters.';
    }

    if (!seoTitle.value.trim()) {
        errors.value.seo_title = 'SEO title is required.';
    } else if (seoTitle.value.trim().length > 255) {
        errors.value.seo_title = 'SEO title must be under 255 characters.';
    }

    if (!seoDescription.value.trim()) {
        errors.value.seo_description = 'SEO description is required.';
    } else if (seoDescription.value.trim().length > 255) {
        errors.value.seo_description = 'SEO description must be under 255 characters.';
    }

    if (!content.value.trim()) {
        errors.value.content = 'Content is required.';
    }

    if (sortOrder.value === '' || sortOrder.value === null) {
        errors.value.sort_order = 'Sort order is required.';
    } else if (Number.isNaN(Number(sortOrder.value)) || Number(sortOrder.value) < 1) {
        errors.value.sort_order = 'Sort order must be 1 or more.';
    }

    return Object.keys(errors.value).length === 0;
}
</script>