<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Product SEO</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ seoEntries.length }} total</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchSeoEntries()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink to="/admin/seo/create"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors ml-2">
                    Add SEO
                </NuxtLink>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by product, title or keyword..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredSeoEntries.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Product</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Meta Title</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Meta Description</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Focus Keyword</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Canonical URL</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Updated</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="entry in filteredSeoEntries" :key="entry.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ entry.product.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ entry.meta_title || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400 max-w-xs truncate">{{ entry.meta_description || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ entry.focus_keyword || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400 max-w-xs truncate">{{ entry.canonical_url || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(entry.created_at) }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ entry.created_at ==
                            entry.updated_at ? '-' : formatDate(entry.updated_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(entry.id, el)">
                            <button type="button" @click="toggleMenu(entry.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === entry.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(entry)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleDelete(entry)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching SEO entries' : 'No SEO entries' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'SEO entries you add will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete SEO entry"
            :message="`Are you sure you want to delete SEO for ${seoToDelete?.product_name}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const seoEntries = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const seoToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const filteredSeoEntries = computed(() => {
    if (!search.value.trim()) return seoEntries.value;
    const query = search.value.trim().toLowerCase();
    return seoEntries.value.filter((entry) =>
        (entry.product_name || '').toLowerCase().includes(query) ||
        (entry.meta_title || '').toLowerCase().includes(query) ||
        (entry.focus_keyword || '').toLowerCase().includes(query)
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

async function fetchSeoEntries() {
    try {
        const data = await authFetch('/api/admin/seo');
        if (data) {
            seoEntries.value = data;
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

function handleEdit(entry) {
    closeMenu();
    navigateTo(`/admin/seo/${entry.id}/edit`);
}

function handleDelete(entry) {
    closeMenu();
    seoToDelete.value = entry;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const entry = seoToDelete.value;
    if (!entry) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting SEO entry...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/seo/${entry.id}`, {
            method: 'DELETE'
        });
        seoEntries.value = seoEntries.value.filter((e) => e.id !== entry.id);
        statusType.value = 'success';
        statusMessage.value = 'SEO entry deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete SEO entry.';
    } finally {
        seoToDelete.value = null;
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

await fetchSeoEntries();
</script>