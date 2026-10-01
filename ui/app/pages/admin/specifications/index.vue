<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Product specifications</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ specifications.length }} total</p>
            </div>
            <NuxtLink to="/admin/specifications/create"
                class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors">
                Add Specifications
            </NuxtLink>
        </div>

        <div class="w-full border border-dark-300 rounded-lg bg-dark-100">
            <table v-if="filteredSpecifications.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Group</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Key</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Value</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Unit</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Order</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Highlighted</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Filterable</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="spec in filteredSpecifications" :key="spec.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-400">{{ spec.group_name || '-' }}</td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ spec.key }}</td>
                        <td class="px-4 py-3 text-zinc-200">{{ spec.value }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ spec.unit || '-' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ spec.sort_order }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="spec.is_highlighted
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ spec.is_highlighted ? 'Yes' : 'No' }}
                            </span>
                        </td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="spec.is_filterable
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ spec.is_filterable ? 'Yes' : 'No' }}
                            </span>
                        </td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="spec.is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ spec.is_active ? 'Active' : 'Inactive' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400">{{ formatDate(spec.created_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(spec.id, el)">
                            <button type="button" @click="toggleMenu(spec.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === spec.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(spec)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleToggleActive(spec)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ spec.is_active ? 'Deactivate' : 'Activate' }}
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">No specifications</p>
                <p class="text-zinc-500 text-sm mt-1">Specifications you add will show up here.</p>
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

const specifications = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const route = useRoute();
search.value = route.query.search || '';

const filteredSpecifications = computed(() => {
    if (!search.value.trim()) return specifications.value;
    const query = search.value.trim().toLowerCase();
    return specifications.value.filter((spec) =>
        spec.id.toLowerCase().includes(query) ||
        spec.product_id.toLowerCase().includes(query) ||
        spec.key.toLowerCase().includes(query) ||
        spec.value.toLowerCase().includes(query) ||
        (spec.group_name || '').toLowerCase().includes(query)
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

async function fetchSpecifications() {
    try {
        const data = await authFetch('/api/admin/specifications');
        if (data) {
            specifications.value = data;
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

function handleEdit(spec) {
    closeMenu();
    navigateTo(`/admin/specifications/${spec.id}/edit`);
}

async function handleToggleActive(spec) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = spec.is_active ? 'Deactivating specification...' : 'Activating specification...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/specifications/${spec.id}`, {
            method: 'PATCH',
            body: { is_active: !spec.is_active }
        });
        spec.is_active = !spec.is_active;
        spec.updated_at = new Date().toISOString();
        statusType.value = 'success';
        statusMessage.value = spec.is_active ? 'Specification activated.' : 'Specification deactivated.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update specification.';
    } finally {
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

await fetchSpecifications();
</script>