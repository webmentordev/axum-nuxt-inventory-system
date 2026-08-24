<template>
    <section class="h-full w-full p-6">
        <div class="max-w-4xl pb-6">
            <h1 class="text-xl font-bold text-white">Create Product</h1>
            <p class="text-sm text-zinc-500 mt-1">Add a new product to the catalog.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Category</label>
                        <AdminSelect v-model="categoryId" :options="categoryOptions"
                            :placeholder="categoriesLoading ? 'Loading categories...' : 'Select a category'" />
                        <p v-if="errors.category_id" class="text-xs text-red-400 mt-1">{{ errors.category_id }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Sub-category</label>
                        <AdminSelect v-model="subCategoryId" :options="subCategoryOptions" :placeholder="subCategoriesLoading
                            ? 'Loading sub-categories...'
                            : (categoryId ? 'Select a sub-category' : 'Select a category first')"
                            :disabled="!categoryId" />
                        <p v-if="errors.sub_category_id" class="text-xs text-red-400 mt-1">{{ errors.sub_category_id }}
                        </p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Brand</label>
                        <AdminSelect v-model="brandId" :options="brandOptions"
                            :placeholder="brandsLoading ? 'Loading brands...' : 'Select a brand'" />
                        <p v-if="errors.brand_id" class="text-xs text-red-400 mt-1">{{ errors.brand_id }}</p>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Name</label>
                        <AdminInput v-model="name" placeholder="e.g. 12V 100Ah Lithium Battery"
                            @update:model-value="onNameInput" />
                        <p v-if="errors.name" class="text-xs text-red-400 mt-1">{{ errors.name }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Slug</label>
                        <AdminInput v-model="slug" placeholder="auto-generated-from-name" />
                        <p v-if="errors.slug" class="text-xs text-red-400 mt-1">{{ errors.slug }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Model</label>
                        <AdminInput v-model="model" placeholder="Optional model number..." />
                        <p v-if="errors.model" class="text-xs text-red-400 mt-1">{{ errors.model }}</p>
                    </div>
                </div>


                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Description</label>
                    <AdminTextarea v-model="description" placeholder="Optional description..." rows="4" />
                    <p v-if="errors.description" class="text-xs text-red-400 mt-1">{{ errors.description }}</p>
                </div>

                <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Power (W)</label>
                        <AdminInput v-model="powerRatingWatts" type="number" step="0.01" placeholder="e.g. 500" />
                        <p v-if="errors.power_rating_watts" class="text-xs text-red-400 mt-1">{{
                            errors.power_rating_watts }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Voltage (V)</label>
                        <AdminInput v-model="voltageRating" type="number" step="0.01" placeholder="e.g. 12" />
                        <p v-if="errors.voltage_rating" class="text-xs text-red-400 mt-1">{{ errors.voltage_rating }}
                        </p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Capacity (Ah)</label>
                        <AdminInput v-model="capacityAh" type="number" step="0.01" placeholder="e.g. 100" />
                        <p v-if="errors.capacity_ah" class="text-xs text-red-400 mt-1">{{ errors.capacity_ah }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Warranty (mo.)</label>
                        <AdminInput v-model="warrantyMonths" type="number" step="1" placeholder="e.g. 24" />
                        <p v-if="errors.warranty_months" class="text-xs text-red-400 mt-1">{{ errors.warranty_months }}
                        </p>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Cost Price</label>
                        <AdminInput v-model="costPrice" type="number" step="0.01" placeholder="e.g. 15000" />
                        <p v-if="errors.cost_price" class="text-xs text-red-400 mt-1">{{ errors.cost_price }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Selling Price</label>
                        <AdminInput v-model="sellingPrice" type="number" step="0.01" placeholder="e.g. 19999" />
                        <p v-if="errors.selling_price" class="text-xs text-red-400 mt-1">{{ errors.selling_price }}</p>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Quantity in Stock</label>
                        <AdminInput v-model="quantityInStock" type="number" step="1" placeholder="0" />
                        <p v-if="errors.quantity_in_stock" class="text-xs text-red-400 mt-1">{{ errors.quantity_in_stock
                            }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Reorder Level</label>
                        <AdminInput v-model="reorderLevel" type="number" step="1" placeholder="0" />
                        <p v-if="errors.reorder_level" class="text-xs text-red-400 mt-1">{{ errors.reorder_level }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Unit</label>
                        <AdminInput v-model="unit" placeholder="e.g. piece" />
                        <p v-if="errors.unit" class="text-xs text-red-400 mt-1">{{ errors.unit }}</p>
                    </div>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Image URL</label>
                    <AdminInput v-model="imageUrl" placeholder="https://..." />
                    <p v-if="errors.image_url" class="text-xs text-red-400 mt-1">{{ errors.image_url }}</p>
                </div>

                <label class="flex items-center gap-2 w-fit cursor-pointer select-none">
                    <input type="checkbox" v-model="isActive"
                        class="h-4 w-4 rounded border-zinc-600 bg-zinc-800 text-lime-main focus:ring-lime-main" />
                    <span class="text-sm font-semibold text-zinc-300">Active</span>
                </label>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Create Product
                </button>
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

const categories = ref([]);
const categoriesLoading = ref(true);

const subCategories = ref([]);
const subCategoriesLoading = ref(false);

const brands = ref([]);
const brandsLoading = ref(true);

const categoryId = ref(null);
const subCategoryId = ref(null);
const brandId = ref(null);

const name = ref('');
const slug = ref('');
const slugTouched = ref(false);
const model = ref('');
const description = ref('');

const powerRatingWatts = ref('');
const voltageRating = ref('');
const capacityAh = ref('');
const warrantyMonths = ref('');

const costPrice = ref('');
const sellingPrice = ref('');

const quantityInStock = ref('0');
const reorderLevel = ref('0');
const unit = ref('piece');

const imageUrl = ref('');
const isActive = ref(true);

const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const categoryOptions = computed(() =>
    categories.value.map((category) => ({
        label: category.is_active ? category.name : `${category.name} (Inactive)`,
        value: category.id
    }))
);

const subCategoryOptions = computed(() =>
    subCategories.value.map((subCategory) => ({
        label: subCategory.is_active ? subCategory.name : `${subCategory.name} (Inactive)`,
        value: subCategory.id
    }))
);

const brandOptions = computed(() =>
    brands.value.map((brand) => ({
        label: brand.is_active ? brand.name : `${brand.name} (Inactive)`,
        value: brand.id
    }))
);

function slugify(value) {
    return value
        .trim()
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '-')
        .replace(/(^-|-$)/g, '');
}

function onNameInput(value) {
    if (!slugTouched.value) {
        slug.value = slugify(value || '');
    }
}

watch(slug, (newValue, oldValue) => {
    if (newValue !== slugify(name.value)) {
        slugTouched.value = true;
    }
});

watch(categoryId, async (newCategoryId) => {
    subCategoryId.value = null;
    subCategories.value = [];
    if (!newCategoryId) return;
    await fetchSubCategories(newCategoryId);
});

async function fetchCategories() {
    categoriesLoading.value = true;
    try {
        const data = await authFetch('/api/admin/categories');
        if (data) {
            categories.value = data;
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to load categories.';
        showStatus.value = true;
    } finally {
        categoriesLoading.value = false;
    }
}

async function fetchSubCategories(categoryIdValue) {
    subCategoriesLoading.value = true;
    try {
        const data = await authFetch('/api/admin/sub-categories/' + categoryIdValue);
        if (data) {
            subCategories.value = data;
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to load sub-categories.';
        showStatus.value = true;
    } finally {
        subCategoriesLoading.value = false;
    }
}

async function fetchBrands() {
    brandsLoading.value = true;
    try {
        const data = await authFetch('/api/admin/brands');
        if (data) {
            brands.value = data;
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to load brands.';
        showStatus.value = true;
    } finally {
        brandsLoading.value = false;
    }
}

function toNumberOrNull(value) {
    if (value === '' || value === null || value === undefined) return null;
    const n = Number(value);
    return Number.isNaN(n) ? null : n;
}

function validate() {
    errors.value = {};

    if (!categoryId.value) {
        errors.value.category_id = 'Category is required.';
    }

    if (!name.value.trim()) {
        errors.value.name = 'Name is required.';
    } else if (name.value.trim().length < 2) {
        errors.value.name = 'Name must be at least 2 characters.';
    }

    if (!slug.value.trim()) {
        errors.value.slug = 'Slug is required.';
    }

    if (description.value && description.value.trim().length > 500) {
        errors.value.description = 'Description must be under 500 characters.';
    }

    if (costPrice.value === '' || Number(costPrice.value) < 0) {
        errors.value.cost_price = 'Cost price is required and must be 0 or more.';
    }

    if (sellingPrice.value === '' || Number(sellingPrice.value) < 0) {
        errors.value.selling_price = 'Selling price is required and must be 0 or more.';
    }

    if (quantityInStock.value === '' || Number(quantityInStock.value) < 0) {
        errors.value.quantity_in_stock = 'Quantity must be 0 or more.';
    }

    if (reorderLevel.value === '' || Number(reorderLevel.value) < 0) {
        errors.value.reorder_level = 'Reorder level must be 0 or more.';
    }

    if (!unit.value.trim()) {
        errors.value.unit = 'Unit is required.';
    }

    if (warrantyMonths.value !== '' && Number(warrantyMonths.value) < 0) {
        errors.value.warranty_months = 'Warranty must be 0 or more.';
    }

    return Object.keys(errors.value).length === 0;
}

function resetForm() {
    categoryId.value = null;
    subCategoryId.value = null;
    brandId.value = null;
    name.value = '';
    slug.value = '';
    slugTouched.value = false;
    model.value = '';
    description.value = '';
    powerRatingWatts.value = '';
    voltageRating.value = '';
    capacityAh.value = '';
    warrantyMonths.value = '';
    costPrice.value = '';
    sellingPrice.value = '';
    quantityInStock.value = '0';
    reorderLevel.value = '0';
    unit.value = 'piece';
    imageUrl.value = '';
    isActive.value = true;
}

async function handleSubmit() {
    if (!validate()) return;

    statusType.value = 'loading';
    statusMessage.value = 'Creating product...';
    showStatus.value = true;

    try {
        const data = await authFetch('/api/admin/products', {
            method: 'POST',
            body: {
                category_id: categoryId.value,
                sub_category_id: subCategoryId.value,
                brand_id: brandId.value,

                name: name.value.trim(),
                slug: slug.value.trim(),
                model: model.value.trim() || null,
                description: description.value.trim() || null,

                power_rating_watts: toNumberOrNull(powerRatingWatts.value),
                voltage_rating: toNumberOrNull(voltageRating.value),
                capacity_ah: toNumberOrNull(capacityAh.value),
                warranty_months: toNumberOrNull(warrantyMonths.value),

                cost_price: toNumberOrNull(costPrice.value),
                selling_price: toNumberOrNull(sellingPrice.value),

                quantity_in_stock: toNumberOrNull(quantityInStock.value) ?? 0,
                reorder_level: toNumberOrNull(reorderLevel.value) ?? 0,
                unit: unit.value.trim(),

                image_url: imageUrl.value.trim() || null,
                is_active: isActive.value
            }
        });

        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Product created.';
            resetForm();
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to create product.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

await Promise.all([fetchCategories(), fetchBrands()]);

</script>