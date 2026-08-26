<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Products</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ products.length }} total</p>
            </div>
            <NuxtLink to="/admin/products/create"
                class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors">
                Add Product
            </NuxtLink>
        </div>

        <div class="w-full border border-dark-300 rounded-lg bg-dark-100">
            <table v-if="filteredProducts.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Preview</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">SKU</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Cost</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Comp. Cost</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Sell Cost</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Sell/C Cost</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Sock</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Created</th>
                        <th class="text-right px-4 py-3 font-semibold text-zinc-400 w-12"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="product in filteredProducts" :key="product.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3">
                            <NuxtLink :to="product.image_url" target="_blank">
                                <img :src="product.image_url" :alt="product.name"
                                    class="w-9 h-9 rounded-lg object-cover border border-dark-300 bg-dark-300" />
                            </NuxtLink>
                        </td>
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ product.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ product.sku }}</td>
                        <td class="px-4 py-3 text-zinc-200">{{ currency }} {{ product.cost_price }}</td>
                        <td class="px-4 py-3 text-zinc-200">{{ currency }} {{ product.compare_at_cost_price }}</td>
                        <td class="px-4 py-3 text-zinc-200">{{ currency }} {{ product.selling_price }}</td>
                        <td class="px-4 py-3 text-zinc-200">{{ currency }} {{ product.compare_at_selling_price }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ product.quantity_in_stock }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="product.is_active
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ product.is_active ? 'Active' : 'Inactive' }}
                            </span>
                        </td>
                        <td class="px-4 py-3 text-zinc-400">{{ formatDate(product.created_at) }}</td>
                        <td class="px-4 py-3 text-right relative" :ref="(el) => setMenuRef(product.id, el)">
                            <button type="button" @click="toggleMenu(product.id)"
                                class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors">
                                <Icon name="mdi:dots-vertical" size="20" />
                            </button>

                            <div v-if="openMenuId === product.id"
                                class="absolute right-4 top-full mt-1 w-40 rounded-lg border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden text-left">
                                <NuxtLink :to='`/products/${product.slug}`'
                                    class="block w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    View
                                </NuxtLink>
                                <button type="button" @click="handleEdit(product)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    Edit
                                </button>
                                <button type="button" @click="handleToggleActive(product)"
                                    class="w-full px-3 py-2 text-sm text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors text-left">
                                    {{ product.is_active ? 'Deactivate' : 'Activate' }}
                                </button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">No products</p>
                <p class="text-zinc-500 text-sm mt-1">Products you add will show up here.</p>
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

const config = useRuntimeConfig().public;

const currency = ref(config.currency);
const products = ref([]);
const search = ref('');
const errors = ref({});
const openMenuId = ref(null);
const menuRefs = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const confirmOpen = ref(false);
const productToDelete = ref(null);

const route = useRoute();
search.value = route.query.search || '';

const filteredProducts = computed(() => {
    if (!search.value.trim()) return products.value;
    const query = search.value.trim().toLowerCase();
    return products.value.filter((product) =>
        product.id.toLowerCase().includes(query) ||
        product.name.toLowerCase().includes(query) ||
        product.slug.toLowerCase().includes(query) ||
        (product.description || '').toLowerCase().includes(query)
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

async function fetchProducts() {
    try {
        const data = await authFetch('/api/admin/products');
        if (data) {
            products.value = data;
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

function handleEdit(product) {
    closeMenu();
    navigateTo(`/admin/products/${product.id}/edit`);
}

async function handleToggleActive(product) {
    closeMenu();
    statusType.value = 'loading';
    statusMessage.value = product.is_active ? 'Deactivating product...' : 'Activating product...';
    showStatus.value = true;

    try {
        await authFetch(`/api/admin/products/${product.id}`, {
            method: 'PATCH',
            body: { is_active: !product.is_active }
        });
        product.is_active = !product.is_active;
        product.updated_at = new Date().toISOString();
        statusType.value = 'success';
        statusMessage.value = product.is_active ? 'Product activated.' : 'Product deactivated.';
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update product.';
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

await fetchProducts();
</script>