<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Info Cards</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ infoCards.length }} total</p>
            </div>
            <div class="flex items-center gap-3">
                <AdminButton @click="fetchInfoCards()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink to="/admin/cards/create"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors">
                    Add info card
                </NuxtLink>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by title, offer title, description or path..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredInfoCards.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Image</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Offer title</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Title</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Description</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Path</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Order</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Updated</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="card in filteredInfoCards" :key="card.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3">
                            <img v-if="card.image_url" :src="card.image_url" :alt="card.title"
                                class="w-9 h-9 rounded-lg object-cover border-2 border-dark-100 bg-dark-300" />
                            <div v-else
                                class="w-9 h-9 rounded-lg border border-dark-300 bg-dark-200 flex items-center justify-center">
                                <Icon name="mdi:image-off-outline" size="16" class="text-zinc-600" />
                            </div>
                        </td>
                        <td class="px-4 py-3 text-zinc-400">{{ card.offer_title }}</td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ card.title }}</td>
                        <td class="px-4 py-3 text-zinc-400 max-w-xs truncate">{{ card.description || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ card.path_url }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ card.sort_order }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="card.is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ card.is_active ? 'Active' : 'Inactive' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(card.created_at) }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ card.created_at ==
                            card.updated_at ? '-' : formatDate(card.updated_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(card.id, el)">
                            <button type="button" @click="toggleMenu(card.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === card.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(card)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleToggleActive(card)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ card.is_active ? 'Deactivate' : 'Activate' }}
                                </button>
                                <button type="button" @click="handleDelete(card)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching info cards' : 'No info cards' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Info cards you add will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete info card"
            :message="`Are you sure you want to delete ${cardToDelete?.title}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const infoCards = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const cardToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const filteredInfoCards = computed(() => {
    if (!search.value.trim()) return infoCards.value;
    const query = search.value.trim().toLowerCase();
    return infoCards.value.filter((card) =>
        card.id.toLowerCase().includes(query) ||
        card.offer_title.toLowerCase().includes(query) ||
        card.title.toLowerCase().includes(query) ||
        (card.description || '').toLowerCase().includes(query) ||
        (card.path_url || '').toLowerCase().includes(query)
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

async function fetchInfoCards() {
    try {
        const data = await authFetch('/api/admin/info-cards');
        if (data) {
            infoCards.value = data;
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

function handleEdit(card) {
    closeMenu();
    navigateTo(`/admin/cards/${card.id}/edit`);
}

async function handleToggleActive(card) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = card.is_active ? 'Deactivating info card...' : 'Activating info card...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/info-cards/${card.id}`, {
            method: 'PATCH',
            body: { is_active: !card.is_active }
        });
        card.is_active = !card.is_active;
        card.updated_at = new Date().toISOString();
        statusType.value = 'success';
        statusMessage.value = card.is_active ? 'Info card activated.' : 'Info card deactivated.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update info card.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

function handleDelete(card) {
    closeMenu();
    cardToDelete.value = card;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const card = cardToDelete.value;
    if (!card) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting info card...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/info-cards/${card.id}`, {
            method: 'DELETE'
        });
        infoCards.value = infoCards.value.filter((c) => c.id !== card.id);
        statusType.value = 'success';
        statusMessage.value = 'Info card deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete info card.';
    } finally {
        cardToDelete.value = null;
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

await fetchInfoCards();
</script>