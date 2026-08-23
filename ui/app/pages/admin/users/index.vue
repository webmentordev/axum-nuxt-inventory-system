<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Users</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ users.length }} total</p>
            </div>
            <div class="flex items-center">
                <AdminButton @click="fetchUsers()" icon="tabler:refresh">Refresh</AdminButton>
            </div>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by name or email..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredUsers.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Email</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Role</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Joined</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Updated</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="user in filteredUsers" :key="user.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ user.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ user.email }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="user.is_admin
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ user.is_admin ? 'Admin' : 'User' }}
                            </span>
                        </td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="user.is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ user.is_active ? 'Active' : 'Inactive' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(user.created_at) }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ user.created_at ==
                            user.updated_at ? '-' : formatDate(user.updated_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(user.id, el)">
                            <button type="button" @click="toggleMenu(user.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === user.id"
                                class="absolute right-4 top-full mt-1 w-44 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleToggleStatus(user)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ user.is_admin ? 'Deactivate user' : 'Activate user' }}
                                </button>
                                <button type="button" @click="handleToggleAdmin(user)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ user.is_admin ? 'Remove admin' : 'Make admin' }}
                                </button>
                                <button type="button" @click="handleDelete(user)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching users' : 'No users' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'Users will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete user"
            :message="`Are you sure you want to delete ${userToDelete?.name}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const users = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const userToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const filteredUsers = computed(() => {
    if (!search.value.trim()) return users.value;
    const query = search.value.trim().toLowerCase();
    return users.value.filter((user) =>
        user.name.toLowerCase().includes(query) ||
        user.email.toLowerCase().includes(query)
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

async function fetchUsers() {
    try {
        const data = await authFetch('/api/admin/users');
        if (data) {
            users.value = data;
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

async function handleToggleAdmin(user) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = user.is_admin ? 'Removing admin access...' : 'Granting admin access...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/users/${user.id}`, {
            method: 'PATCH',
            body: { is_admin: !user.is_admin }
        });
        user.is_admin = !user.is_admin;
        user.updated_at = new Date().toISOString();
        statusType.value = 'success';
        statusMessage.value = user.is_admin ? 'User is now an admin.' : 'Admin access removed.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update user.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

async function handleToggleStatus(user) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = user.is_admin ? 'Deactivating user...' : 'Activating user...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/users/${user.id}`, {
            method: 'PATCH',
            body: { is_admin: !user.is_admin }
        });
        user.is_active = !user.is_active;
        user.updated_at = new Date().toISOString();
        statusType.value = 'success';
        statusMessage.value = user.is_active ? 'User activated.' : 'User deactivated.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update status user.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}


function handleDelete(user) {
    closeMenu();
    userToDelete.value = user;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const user = userToDelete.value;
    if (!user) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting user...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/users/${user.id}`, {
            method: 'DELETE'
        });
        users.value = users.value.filter((u) => u.id !== user.id);
        statusType.value = 'success';
        statusMessage.value = 'User deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete user.';
    } finally {
        userToDelete.value = null;
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

await fetchUsers();
</script>