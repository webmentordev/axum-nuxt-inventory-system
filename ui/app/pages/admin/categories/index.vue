<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Categories</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ categories.length }} total</p>
            </div>
            <NuxtLink to="/admin/categories/create"
                class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors">
                Add category
            </NuxtLink>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by name, slug or description..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredCategories.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Images</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Slug</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Description</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Products</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Sub-categories</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="category in filteredCategories" :key="category.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3">
                            <div v-if="category.images && category.images.length" class="flex items-center -space-x-3">
                                <img v-for="(image, index) in category.images.slice(0, 3)" :key="image.id"
                                    :src="image.file_path" :alt="image.name || category.name"
                                    class="w-9 h-9 rounded-lg object-cover border-2 border-dark-100 bg-dark-300"
                                    :style="{ zIndex: category.images.length - index }" />
                                <span v-if="category.images.length > 3"
                                    class="w-9 h-9 rounded-lg border-2 border-dark-100 bg-dark-300 text-zinc-300 text-xs font-semibold flex items-center justify-center">
                                    +{{ category.images.length - 3 }}
                                </span>
                            </div>
                            <div v-else
                                class="w-9 h-9 rounded-lg border border-dark-300 bg-dark-200 flex items-center justify-center">
                                <Icon name="mdi:image-off-outline" size="16" class="text-zinc-600" />
                            </div>
                        </td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ category.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ category.slug }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ category.description || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ category.products_count }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ category.sub_categories_count }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="category.is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ category.is_active ? 'Active' : 'Inactive' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(category.id, el)">
                            <button type="button" @click="toggleMenu(category.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === category.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(category)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleToggleActive(category)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ category.is_active ? 'Deactivate' : 'Activate' }}
                                </button>
                                <button type="button" @click="handleDelete(category)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching categories' : 'No categories' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Categories you add will show up here.' }}
                </p>
            </div>
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
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const filteredCategories = computed(() => {
    if (!search.value.trim()) return categories.value;
    const query = search.value.trim().toLowerCase();
    return categories.value.filter((category) =>
        category.name.toLowerCase().includes(query) ||
        category.slug.toLowerCase().includes(query) ||
        (category.description || '').toLowerCase().includes(query)
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

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

async function fetchCategories() {
    try {
        const data = await authFetch('/api/categories');
        if (data) {
            categories.value = data;
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

function handleEdit(category) {
    closeMenu();
    navigateTo(`/admin/categories/${category.id}/edit`);
}

async function handleToggleActive(category) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = category.is_active ? 'Deactivating category...' : 'Activating category...';
    showStatus.value = true;

    try {
        await authFetch(`/api/categories/${category.id}`, {
            method: 'PATCH',
            body: { is_active: !category.is_active }
        });
        category.is_active = !category.is_active;
        statusType.value = 'success';
        statusMessage.value = category.is_active ? 'Category activated.' : 'Category deactivated.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update category.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

async function handleDelete(category) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = 'Deleting category...';
    showStatus.value = true;

    try {
        await authFetch(`/api/categories/${category.id}`, {
            method: 'DELETE'
        });
        categories.value = categories.value.filter((c) => c.id !== category.id);
        statusType.value = 'success';
        statusMessage.value = 'Category deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete category.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

onMounted(fetchCategories);
</script>