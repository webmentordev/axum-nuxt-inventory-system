<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Sub-categories</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ subCategories.length }} total</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchSubCategories()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink to="/admin/sub-categories/create"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors ml-2">
                    Add sub-category
                </NuxtLink>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by name, slug or description..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredSubCategories.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Images</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Slug</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Description</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Category</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Products</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Updated</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="subCategory in filteredSubCategories" :key="subCategory.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3">
                            <div v-if="subCategory.uploads && subCategory.uploads.length"
                                class="flex items-center -space-x-3">
                                <img v-for="(image, index) in subCategory.uploads.slice(0, 3)" :key="image.id"
                                    :src="image.file_path" :alt="image.name || subCategory.name"
                                    class="w-9 h-9 rounded-lg object-cover border-2 border-dark-100 bg-dark-300"
                                    :style="{ zIndex: subCategory.uploads.length - index }" />
                                <span v-if="subCategory.uploads.length > 3"
                                    class="w-9 h-9 rounded-lg border-2 border-dark-100 bg-dark-300 text-zinc-300 text-xs font-semibold flex items-center justify-center">
                                    +{{ category.uploads.length - 3 }}
                                </span>
                            </div>
                            <div v-else
                                class="w-9 h-9 rounded-lg border border-dark-300 bg-dark-200 flex items-center justify-center">
                                <Icon name="mdi:image-off-outline" size="16" class="text-zinc-600" />
                            </div>
                        </td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ subCategory.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ subCategory.slug }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ subCategory.description || '—' }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="subCategory.category_is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ subCategory.category.name }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400">{{ subCategory.products_count }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="subCategory.is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ subCategory.is_active ? 'Active' : 'Inactive' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(subCategory.created_at) }}
                        </td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ subCategory.created_at ==
                            subCategory.updated_at ? '-' : formatDate(subCategory.updated_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(subCategory.id, el)">
                            <button type="button" @click="toggleMenu(subCategory.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === subCategory.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(subCategory)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleToggleActive(subCategory)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ subCategory.is_active ? 'Deactivate' : 'Activate' }}
                                </button>
                                <button type="button" @click="handleDelete(subCategory)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching sub-categories' : 'No sub-categories' }}
                </p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Sub-categories you add will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete sub-category"
            :message="`Are you sure you want to delete ${subCategoryToDelete?.name}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const subCategories = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const subCategoryToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const filteredSubCategories = computed(() => {
    if (!search.value.trim()) return subCategories.value;
    const query = search.value.trim().toLowerCase();
    return subCategories.value.filter((subCategory) =>
        subCategory.id.toLowerCase().includes(query) ||
        subCategory.name.toLowerCase().includes(query) ||
        subCategory.slug.toLowerCase().includes(query) ||
        (subCategory.description || '').toLowerCase().includes(query)
    );
});

function setMenuRef(id, el) {
    if (el) {
        menuRefs.value[id] = el;
    } else {
        delete menuRefs.value[id];
    }
}

const activeMenuEl = computed(() => menuRefs.value[openMenuId.value] || null);

onClickOutside(activeMenuEl, () => {
    closeMenu();
});

async function fetchSubCategories() {
    try {
        const data = await authFetch('/api/admin/sub-categories');
        if (data) {
            subCategories.value = data;
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

function toggleMenu(id) {
    openMenuId.value = openMenuId.value === id ? null : id;
}

function closeMenu() {
    openMenuId.value = null;
}

function handleEdit(subCategory) {
    closeMenu();
    navigateTo(`/admin/sub-categories/${subCategory.id}/edit`);
}

async function handleToggleActive(subCategory) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = subCategory.is_active ? 'Deactivating sub-category...' : 'Activating sub-category...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/sub-categories/${subCategory.id}`, {
            method: 'PATCH',
            body: { is_active: !subCategory.is_active }
        });
        subCategory.is_active = !subCategory.is_active;
        subCategory.updated_at = new Date().toISOString();
        statusType.value = 'success';
        statusMessage.value = subCategory.is_active ? 'Sub-category activated.' : 'Sub-category deactivated.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update sub-category.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

function handleDelete(subCategory) {
    closeMenu();
    subCategoryToDelete.value = subCategory;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const subCategory = subCategoryToDelete.value;
    if (!subCategory) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting sub-category...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/sub-categories/${subCategory.id}`, {
            method: 'DELETE'
        });
        subCategories.value = subCategories.value.filter((sc) => sc.id !== subCategory.id);
        statusType.value = 'success';
        statusMessage.value = 'Sub-category deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete sub-category.';
    } finally {
        subCategoryToDelete.value = null;
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

function formatDate(utcString) {
    return new Date(utcString).toLocaleString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: 'numeric',
        minute: '2-digit'
    });
}

await fetchSubCategories();
</script>