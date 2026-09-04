<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Audit Logs</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ logs.length }} shown</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchLogs()" icon="tabler:refresh">Refresh</AdminButton>
            </div>
        </div>

        <div class="mb-4 flex flex-wrap items-center gap-3">
            <div class="max-w-sm w-full">
                <AdminInput v-model="search" placeholder="Search by entity, action, status or user id..." />
            </div>

            <select v-model="entityType" @change="fetchLogs()"
                class="bg-dark-100 border border-dark-300 rounded-md text-sm text-zinc-300 px-3 py-2 focus:outline-none">
                <option value="">All entities</option>
                <option v-for="type in entityTypes" :key="type" :value="type">{{ type }}</option>
            </select>

            <select v-model="action" @change="fetchLogs()"
                class="bg-dark-100 border border-dark-300 rounded-md text-sm text-zinc-300 px-3 py-2 focus:outline-none">
                <option value="">All actions</option>
                <option value="create">Create</option>
                <option value="update">Update</option>
                <option value="delete">Delete</option>
            </select>
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredLogs.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Action</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Entity</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Entity ID</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">User ID</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Details</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="log in filteredLogs" :key="log.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="actionBadgeClass(log.action)">
                                {{ log.action }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ log.entity_type }}</td>
                        <td class="px-4 py-3 text-zinc-400 font-mono text-xs">{{ log.entity_id || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ log.status }}</td>
                        <td class="px-4 py-3 text-zinc-400 font-mono text-xs">{{ log.user_id || '—' }}</td>
                        <td class="px-4 py-3 text-zinc-400 max-w-xs truncate" :title="formatDetails(log.details)">
                            {{ formatDetails(log.details) }}
                        </td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(log.created_at) }}</td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching logs' : 'No logs' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Activity will show up here.' }}
                </p>
            </div>
        </div>

        <div class="flex items-center justify-between mt-4" v-if="logs.length">
            <AdminButton :disabled="offset === 0" @click="prevPage()" icon="tabler:chevron-left">Prev</AdminButton>
            <span class="text-sm text-zinc-500">Page {{ Math.floor(offset / limit) + 1 }}</span>
            <AdminButton :disabled="logs.length < limit" @click="nextPage()" icon="tabler:chevron-right">Next
            </AdminButton>
        </div>
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const logs = ref([]);
const search = ref('');
const entityType = ref('');
const action = ref('');
const limit = ref(50);
const offset = ref(0);

const entityTypes = ['brand', 'category', 'sub_category', 'product', 'product_seo', 'upload', 'contact'];

const route = useRoute();
search.value = route.query.search || '';

const filteredLogs = computed(() => {
    if (!search.value.trim()) return logs.value;
    const query = search.value.trim().toLowerCase();
    return logs.value.filter((log) =>
        (log.entity_type || '').toLowerCase().includes(query) ||
        (log.action || '').toLowerCase().includes(query) ||
        (log.status || '').toLowerCase().includes(query) ||
        (log.user_id || '').toLowerCase().includes(query) ||
        (log.entity_id || '').toLowerCase().includes(query)
    );
});

async function fetchLogs() {
    try {
        const params = new URLSearchParams();
        params.set('limit', limit.value);
        params.set('offset', offset.value);
        if (entityType.value) params.set('entity_type', entityType.value);
        if (action.value) params.set('action', action.value);

        const data = await authFetch(`/api/admin/logs?${params.toString()}`);
        if (data) {
            logs.value = data;
        }
    } catch (e) {
        console.error(e);
    }
}

function nextPage() {
    offset.value += limit.value;
    fetchLogs();
}

function prevPage() {
    offset.value = Math.max(0, offset.value - limit.value);
    fetchLogs();
}

function actionBadgeClass(action) {
    switch (action) {
        case 'create':
            return 'bg-lime-bg text-lime-main';
        case 'update':
            return 'bg-blue-500/10 text-blue-400';
        case 'delete':
            return 'bg-red-500/10 text-red-400';
        default:
            return 'bg-dark-300 text-zinc-400';
    }
}

function formatDetails(details) {
    if (!details) return '—';
    try {
        return typeof details === 'string' ? details : JSON.stringify(details);
    } catch {
        return '—';
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

await fetchLogs();
</script>