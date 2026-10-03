<template>
    <section class="h-full w-full p-6">
        <div class="max-w-3xl">
            <h1 class="text-xl font-bold text-white">Add Specifications</h1>
            <p class="text-sm text-zinc-500 mt-1">Add one or more specifications to a product.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div class="max-w-lg">
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Product</label>
                    <AdminSelect v-model="productId" :options="productOptions"
                        :placeholder="productsLoading ? 'Loading products...' : 'Select a product'" />
                    <p v-if="errors.product" class="text-xs text-red-400 mt-1">{{ errors.product }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Quick Add</label>
                    <div class="flex flex-wrap gap-2">
                        <button v-for="preset in presets" :key="preset.key" type="button" @click="addPreset(preset)"
                            :disabled="isPresetAdded(preset)"
                            class="px-3 py-1.5 rounded-md text-xs font-semibold border border-dark-300 text-zinc-300 hover:bg-dark-200 transition-colors disabled:opacity-40 disabled:cursor-not-allowed">
                            + {{ preset.key }}
                        </button>
                    </div>
                </div>

                <div v-for="(row, index) in rows" :key="index"
                    class="border border-dark-300 bg-dark-100 rounded-lg p-4 flex flex-col gap-4">
                    <div class="flex items-center justify-between">
                        <p class="text-sm font-semibold text-zinc-300">Specification {{ index + 1 }}</p>
                        <button type="button" @click="removeRow(index)" :disabled="rows.length === 1"
                            class="p-1.5 rounded-md text-zinc-400 hover:text-white hover:bg-dark-300 transition-colors disabled:opacity-40 disabled:cursor-not-allowed">
                            <Icon name="mdi:close" size="18" />
                        </button>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Group</label>
                            <AdminInput v-model="row.group_name" placeholder="e.g. Electrical" />
                        </div>
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Key</label>
                            <AdminInput v-model="row.key" placeholder="e.g. Maximum Power" />
                            <p v-if="rowErrors[index]?.key" class="text-xs text-red-400 mt-1">{{ rowErrors[index].key }}
                            </p>
                        </div>
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Value</label>
                            <AdminInput v-model="row.value" placeholder="e.g. 550" />
                            <p v-if="rowErrors[index]?.value" class="text-xs text-red-400 mt-1">{{
                                rowErrors[index].value }}</p>
                        </div>
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Unit</label>
                            <AdminInput v-model="row.unit" placeholder="e.g. W" />
                        </div>
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Sort Order</label>
                            <input v-model.number="row.sort_order" type="number" min="0"
                                class="w-full px-3 py-2 rounded-md text-sm bg-dark-200 border border-dark-300 text-zinc-200 focus:outline-none focus:border-lime-main/50" />
                        </div>
                    </div>

                    <div class="flex flex-wrap items-center gap-6">
                        <label class="flex items-center gap-2 text-sm text-zinc-300 cursor-pointer">
                            <input v-model="row.is_highlighted" type="checkbox" class="accent-lime-500" />
                            Highlighted
                        </label>
                        <label class="flex items-center gap-2 text-sm text-zinc-300 cursor-pointer">
                            <input v-model="row.is_filterable" type="checkbox" class="accent-lime-500" />
                            Filterable
                        </label>
                        <label class="flex items-center gap-2 text-sm text-zinc-300 cursor-pointer">
                            <input v-model="row.is_active" type="checkbox" class="accent-lime-500" />
                            Active
                        </label>
                    </div>
                </div>

                <div class="flex items-center gap-3 mt-2">
                    <button type="button" @click="addRow"
                        class="px-4 py-2 rounded-md text-sm font-semibold border border-dark-300 text-zinc-300 hover:bg-dark-200 transition-colors">
                        Add Another
                    </button>
                    <button type="submit" :disabled="submitting"
                        class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
                        {{ submitting ? 'Saving...' : 'Save Specifications' }}
                    </button>
                </div>
            </form>
        </div>

        <AdminStatusCard v-model="showStatus" :type="statusType" :message="statusMessage" />
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});

const { authFetch } = useAuthFetch();

function newRow(sortOrder = 0) {
    return {
        group_name: '',
        key: '',
        value: '',
        unit: '',
        sort_order: sortOrder,
        is_highlighted: false,
        is_filterable: false,
        is_active: true
    };
}

const productId = ref(null);
const productOptions = ref([]);
const productsLoading = ref(false);

const rows = ref([newRow(0)]);
const errors = ref({});
const rowErrors = ref({});
const submitting = ref(false);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const presets = [
    { group_name: 'Electrical', key: 'Power Rating', unit: 'W', is_highlighted: true, is_filterable: true },
    { group_name: 'Electrical', key: 'Voltage Rating', unit: 'V', is_highlighted: false, is_filterable: true },
    { group_name: 'Battery', key: 'Capacity', unit: 'Ah', is_highlighted: false, is_filterable: true },
    { group_name: 'Battery', key: 'Energy', unit: 'kWh', is_highlighted: false, is_filterable: true },
    { group_name: 'Pricing', key: 'Price Per Watt', unit: 'PKR/W', is_highlighted: false, is_filterable: false }
];

function addRow() {
    rows.value.push(newRow(rows.value.length));
}

function removeRow(index) {
    if (rows.value.length === 1) return;
    rows.value.splice(index, 1);
}

async function fetchProducts() {
    productsLoading.value = true;
    try {
        const data = await authFetch('/api/admin/products/list');
        if (data) {
            productOptions.value = data.map((item) => ({
                label: `${item.name} (${item.sku})`,
                value: item.id
            }));
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Failed to load products.';
    } finally {
        productsLoading.value = false;
    }
}

function validate() {
    errors.value = {};
    rowErrors.value = {};

    if (!productId.value) {
        errors.value.product = 'Product is required.';
    }

    rows.value.forEach((row, index) => {
        const rowError = {};
        if (!row.key.trim()) rowError.key = 'Key is required.';
        if (!row.value.trim()) rowError.value = 'Value is required.';
        if (Object.keys(rowError).length) rowErrors.value[index] = rowError;
    });

    return Object.keys(errors.value).length === 0 && Object.keys(rowErrors.value).length === 0;
}

function isPresetAdded(preset) {
    return rows.value.some((row) => row.key.trim().toLowerCase() === preset.key.toLowerCase());
}

function addPreset(preset) {
    if (isPresetAdded(preset)) return;

    const first = rows.value[0];
    const firstIsEmpty = rows.value.length === 1 && !first.key.trim() && !first.value.trim();
    const row = {
        ...newRow(firstIsEmpty ? 0 : rows.value.length),
        group_name: preset.group_name,
        key: preset.key,
        unit: preset.unit,
        is_highlighted: preset.is_highlighted,
        is_filterable: preset.is_filterable
    };

    if (firstIsEmpty) {
        rows.value[0] = row;
    } else {
        rows.value.push(row);
    }
}

async function handleSubmit() {
    if (submitting.value) return;
    if (!validate()) return;

    submitting.value = true;
    statusType.value = 'loading';
    statusMessage.value = 'Saving specifications...';
    showStatus.value = true;

    try {
        const specifications = rows.value.map((row, index) => ({
            group_name: row.group_name.trim() || null,
            key: row.key.trim(),
            value: row.value.trim(),
            unit: row.unit.trim() || null,
            sort_order: Number.isInteger(row.sort_order) ? row.sort_order : index,
            is_highlighted: row.is_highlighted,
            is_filterable: row.is_filterable,
            is_active: row.is_active
        }));

        const data = await authFetch('/api/admin/specifications', {
            method: 'POST',
            body: {
                product_id: productId.value,
                specifications
            }
        });

        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Specifications saved.';
            rows.value = [newRow(0)];
            errors.value = {};
            rowErrors.value = {};
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to save specifications.';
    } finally {
        submitting.value = false;
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

await fetchProducts();
</script>