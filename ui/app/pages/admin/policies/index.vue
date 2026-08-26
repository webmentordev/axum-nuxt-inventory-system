<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Policies</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ policies.length }} total</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchPolicies()" icon="tabler:refresh">Refresh</AdminButton>
                <NuxtLink to="/admin/policies/create"
                    class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors ml-2">
                    Add policy
                </NuxtLink>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by name, slug or SEO title..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredPolicies.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Order</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Slug</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">SEO Title</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Updated</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="policy in filteredPolicies" :key="policy.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-400">{{ policy.sort_order }}</td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ policy.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ policy.slug }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ policy.seo_title || '—' }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="policy.is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ policy.is_active ? 'Active' : 'Inactive' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(policy.created_at) }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ policy.created_at ==
                            policy.updated_at ? '-' : formatDate(policy.updated_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(policy.id, el)">
                            <button type="button" @click="toggleMenu(policy.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === policy.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleEdit(policy)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleToggleActive(policy)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ policy.is_active ? 'Deactivate' : 'Activate' }}
                                </button>
                                <button type="button" @click="handleDelete(policy)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching policies' : 'No policies' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Policies you add will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete policy"
            :message="`Are you sure you want to delete ${policyToDelete?.name}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const policies = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const policyToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const filteredPolicies = computed(() => {
    if (!search.value.trim()) return policies.value;
    const query = search.value.trim().toLowerCase();
    return policies.value.filter((policy) =>
        policy.id.toLowerCase().includes(query) ||
        policy.name.toLowerCase().includes(query) ||
        policy.slug.toLowerCase().includes(query) ||
        (policy.seo_title || '').toLowerCase().includes(query)
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

async function fetchPolicies() {
    try {
        const data = await authFetch('/api/admin/policies');
        if (data) {
            policies.value = data;
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

function handleEdit(policy) {
    closeMenu();
    navigateTo(`/admin/policies/${policy.id}/edit`);
}

async function handleToggleActive(policy) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = policy.is_active ? 'Deactivating policy...' : 'Activating policy...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/policies/${policy.id}`, {
            method: 'PATCH',
            body: { is_active: !policy.is_active }
        });
        policy.is_active = !policy.is_active;
        policy.updated_at = new Date().toISOString();
        statusType.value = 'success';
        statusMessage.value = policy.is_active ? 'Policy activated.' : 'Policy deactivated.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update policy.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

function handleDelete(policy) {
    closeMenu();
    policyToDelete.value = policy;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const policy = policyToDelete.value;
    if (!policy) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting policy...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/policies/${policy.id}`, {
            method: 'DELETE'
        });
        policies.value = policies.value.filter((p) => p.id !== policy.id);
        statusType.value = 'success';
        statusMessage.value = 'Policy deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete policy.';
    } finally {
        policyToDelete.value = null;
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

await fetchPolicies();
</script>