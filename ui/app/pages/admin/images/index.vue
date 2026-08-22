<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Images</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ images.length }} total</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchImages()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink to="/admin/images/create"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors ml-2">
                    Add image
                </NuxtLink>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by name, product, category or brand..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredImages.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Preview</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Product</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Category</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Sub Category</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Brand</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="image in filteredImages" :key="image.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3">
                            <NuxtLink :to="image.file_path" target="_blank">
                                <img :src="image.file_path" :alt="image.name"
                                    class="w-9 h-9 rounded-lg object-cover border border-dark-300 bg-dark-300" />
                            </NuxtLink>
                        </td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ image.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ image.product?.name || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ image.category?.name || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ image.sub_category?.name || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ image.brand?.name || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(image.created_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(image.id, el)">
                            <button type="button" @click="toggleMenu(image.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === image.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(image)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleDelete(image)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching images' : 'No images' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Images you add will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete image"
            :message="`Are you sure you want to delete ${imageToDelete?.name}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const images = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const imageToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const filteredImages = computed(() => {
    if (!search.value.trim()) return images.value;
    const query = search.value.trim().toLowerCase();
    return images.value.filter((image) =>
        image.name.toLowerCase().includes(query) ||
        (image.product?.name || '').toLowerCase().includes(query) ||
        (image.category?.name || '').toLowerCase().includes(query) ||
        (image.sub_category?.name || '').toLowerCase().includes(query) ||
        (image.brand?.name || '').toLowerCase().includes(query)
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

async function fetchImages() {
    try {
        const data = await authFetch('/api/admin/images');
        if (data) {
            images.value = data;
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

function handleEdit(image) {
    closeMenu();
    navigateTo(`/admin/images/${image.id}/edit`);
}

function handleDelete(image) {
    closeMenu();
    imageToDelete.value = image;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const image = imageToDelete.value;
    if (!image) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting image...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/images/${image.id}`, {
            method: 'DELETE'
        });
        images.value = images.value.filter((i) => i.id !== image.id);
        statusType.value = 'success';
        statusMessage.value = 'Image deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete image.';
    } finally {
        imageToDelete.value = null;
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

await fetchImages();
</script>