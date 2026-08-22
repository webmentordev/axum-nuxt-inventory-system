<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Contacts</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ contacts.length }} total</p>
            </div>
            <AdminButton @click="fetchContacts()" icon="tabler:refresh">Refresh</AdminButton>
        </div>

        <div class="mb-4 max-w-sm">
            <AdminInput v-model="search" placeholder="Search by name, slug or description..." />
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-visible bg-dark-100">
            <table v-if="filteredcontacts.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Email</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Subject</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Message</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Received</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="contact in filteredcontacts" :key="contact.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ contact.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ contact.email }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ contact.subject }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ contact.message }}</td>
                        <td class="px-4 py-3 text-zinc-400 whitespace-nowrap">{{ formatDate(contact.created_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(contact.id, el)">
                            <button type="button" @click="toggleMenu(contact.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>
                            <div v-if="openMenuId === contact.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <button type="button" @click="handleDelete(contact)"
                                    class="w-full px-3 py-2 text-sm text-red-400 hover:bg-dark-300 hover:text-red-300 transition-colors text-left">
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">{{ search ? 'No matching contacts' : 'No contacts' }}</p>
                <p class="text-zinc-500 text-sm mt-1">
                    {{ search ? 'Try a different search term.' : 'contacts you add will show up here.' }}
                </p>
            </div>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
        <AdminConfirmModal v-model="confirmOpen" title="Delete contact"
            :message="`Are you sure you want to delete the message from ${contactToDelete?.name}? This cannot be undone.`"
            @confirm="confirmDelete" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
const { authFetch } = useAuthFetch();

const contacts = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const route = useRoute();
search.value = route.query.search || '';

const filteredcontacts = computed(() => {
    if (!search.value.trim()) return contacts.value;
    const query = search.value.trim().toLowerCase();
    return contacts.value.filter((contact) =>
        contact.id.toLowerCase().includes(query) ||
        contact.name.toLowerCase().includes(query) ||
        contact.email.toLowerCase().includes(query) ||
        contact.subject.toLowerCase().includes(query) ||
        (contact.message).toLowerCase().includes(query)
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

async function fetchContacts() {
    try {
        const data = await authFetch('/api/admin/contacts');
        if (data) {
            contacts.value = data;
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

const confirmOpen = ref(false);
const contactToDelete = ref(null);

function handleDelete(contact) {
    closeMenu();
    contactToDelete.value = contact;
    confirmOpen.value = true;
}

async function confirmDelete() {
    const contact = contactToDelete.value;
    if (!contact) return;

    statusType.value = 'loading';
    statusMessage.value = 'Deleting contact...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/contacts/${contact.id}`, {
            method: 'DELETE'
        });
        contacts.value = contacts.value.filter((c) => c.id !== contact.id);
        statusType.value = 'success';
        statusMessage.value = 'Contact deleted.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to delete contact.';
    } finally {
        contactToDelete.value = null;
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

await fetchContacts();
</script>