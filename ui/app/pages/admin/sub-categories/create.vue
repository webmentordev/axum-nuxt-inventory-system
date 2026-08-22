<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Create Sub-category</h1>
            <p class="text-sm text-zinc-500 mt-1">Add a new sub-category under an existing category.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Category</label>
                    <AdminSelect v-model="categoryId" :options="categoryOptions"
                        :placeholder="categoriesLoading ? 'Loading categories...' : 'Select a category'" />
                    <p v-if="errors.category_id" class="text-xs text-red-400 mt-1">{{ errors.category_id }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Name</label>
                    <AdminInput v-model="name" placeholder="e.g. Inverters" />
                    <p v-if="errors.name" class="text-xs text-red-400 mt-1">{{ errors.name }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Description</label>
                    <AdminTextarea v-model="description" placeholder="Optional description..." rows="4" />
                    <p v-if="errors.description" class="text-xs text-red-400 mt-1">{{ errors.description }}</p>
                </div>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Create Sub-category
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

const categories = ref([]);
const categoriesLoading = ref(true);

const categoryId = ref(null);
const name = ref('');
const description = ref('');
const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const categoryOptions = computed(() =>
    categories.value.map((category) => ({
        label: category.is_active ? category.name : `${category.name} (Inactive)`,
        value: category.id
    }))
);

async function fetchCategories() {
    categoriesLoading.value = true;
    try {
        const data = await authFetch('/api/admin/categories');
        if (data) {
            categories.value = data;
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Failed to load categories.';
    } finally {
        categoriesLoading.value = false;
    }
}

function validate() {
    errors.value = {};

    if (!categoryId.value) {
        errors.value.category_id = 'Category is required.';
    }

    if (!name.value.trim()) {
        errors.value.name = 'Name is required.';
    } else if (name.value.trim().length < 2) {
        errors.value.name = 'Name must be at least 2 characters.';
    }

    if (!description.value) {
        errors.value.description = 'Description is required.';
    } else if (description.value.trim().length > 500) {
        errors.value.description = 'Description must be under 500 characters.';
    }

    return Object.keys(errors.value).length === 0;
}

async function handleSubmit() {
    if (!validate()) return;

    statusType.value = 'loading';
    statusMessage.value = 'Creating sub-category...';
    showStatus.value = true;

    try {
        const data = await authFetch('/api/admin/sub-categories', {
            method: 'POST',
            body: {
                category_id: categoryId.value,
                name: name.value.trim(),
                description: description.value.trim()
            }
        });

        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Sub-category created.';
            categoryId.value = null;
            name.value = '';
            description.value = '';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to create sub-category.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

await fetchCategories();
</script>