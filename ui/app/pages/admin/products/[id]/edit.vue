<template>
    <section class="h-full w-full p-6">
        <div class="max-w-5xl pb-6">
            <h1 class="text-xl font-bold text-white">Edit Product</h1>
            <p class="text-sm text-zinc-500 mt-1">Update an existing product in the catalog.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
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

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Product Type</label>
                        <AdminSelect v-model="productType" :options="productTypeOptions" placeholder="Select a type" />
                        <p v-if="errors.product_type" class="text-xs text-red-400 mt-1">{{ errors.product_type }}</p>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
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

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Unit</label>
                        <AdminInput v-model="unit" placeholder="e.g. piece" />
                        <p v-if="errors.unit" class="text-xs text-red-400 mt-1">{{ errors.unit }}</p>
                    </div>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Description</label>
                    <AdminTextarea v-model="description" placeholder="Optional description..." rows="4" />
                    <p v-if="errors.description" class="text-xs text-red-400 mt-1">{{ errors.description }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Content</label>
                    <AdminTextarea v-model="content" placeholder="Optional long-form content..." rows="6" />
                    <p v-if="errors.content" class="text-xs text-red-400 mt-1">{{ errors.content }}</p>
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

                <div v-if="productType === 'solar'" class="flex flex-col gap-4">
                    <h2 class="text-sm font-bold text-zinc-400 uppercase tracking-wide mt-2">Solar Panel Specs</h2>

                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Panel Type</label>
                            <AdminInput v-model="panelType" placeholder="e.g. Monocrystalline" />
                            <p v-if="errors.panel_type" class="text-xs text-red-400 mt-1">{{ errors.panel_type }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Cell Type</label>
                            <AdminInput v-model="cellType" placeholder="e.g. PERC" />
                            <p v-if="errors.cell_type" class="text-xs text-red-400 mt-1">{{ errors.cell_type }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Number of Cells</label>
                            <AdminInput v-model="numberOfCells" type="number" step="1" placeholder="e.g. 144" />
                            <p v-if="errors.number_of_cells" class="text-xs text-red-400 mt-1">{{
                                errors.number_of_cells }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Efficiency (%)</label>
                            <AdminInput v-model="efficiencyPercentage" type="number" step="0.01"
                                placeholder="e.g. 21.5" />
                            <p v-if="errors.efficiency_percentage" class="text-xs text-red-400 mt-1">{{
                                errors.efficiency_percentage }}</p>
                        </div>
                    </div>

                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Max System Voltage</label>
                            <AdminInput v-model="maxSystemVoltage" type="number" step="0.01" placeholder="e.g. 1500" />
                            <p v-if="errors.max_system_voltage" class="text-xs text-red-400 mt-1">{{
                                errors.max_system_voltage }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Open Circuit Voltage</label>
                            <AdminInput v-model="openCircuitVoltage" type="number" step="0.01"
                                placeholder="e.g. 49.5" />
                            <p v-if="errors.open_circuit_voltage" class="text-xs text-red-400 mt-1">{{
                                errors.open_circuit_voltage }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Short Circuit Current</label>
                            <AdminInput v-model="shortCircuitCurrent" type="number" step="0.01"
                                placeholder="e.g. 10.5" />
                            <p v-if="errors.short_circuit_current" class="text-xs text-red-400 mt-1">{{
                                errors.short_circuit_current }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Max Power Voltage</label>
                            <AdminInput v-model="maxPowerVoltage" type="number" step="0.01" placeholder="e.g. 41.2" />
                            <p v-if="errors.max_power_voltage" class="text-xs text-red-400 mt-1">{{
                                errors.max_power_voltage }}</p>
                        </div>
                    </div>

                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Max Power Current</label>
                            <AdminInput v-model="maxPowerCurrent" type="number" step="0.01" placeholder="e.g. 9.8" />
                            <p v-if="errors.max_power_current" class="text-xs text-red-400 mt-1">{{
                                errors.max_power_current }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Temp. Coefficient</label>
                            <AdminInput v-model="temperatureCoefficient" type="number" step="0.001"
                                placeholder="e.g. -0.35" />
                            <p v-if="errors.temperature_coefficient" class="text-xs text-red-400 mt-1">{{
                                errors.temperature_coefficient }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Frame Material</label>
                            <AdminInput v-model="frameMaterial" placeholder="e.g. Anodized Aluminum" />
                            <p v-if="errors.frame_material" class="text-xs text-red-400 mt-1">{{ errors.frame_material
                                }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Glass Type</label>
                            <AdminInput v-model="glassType" placeholder="e.g. Tempered" />
                            <p v-if="errors.glass_type" class="text-xs text-red-400 mt-1">{{ errors.glass_type }}</p>
                        </div>
                    </div>

                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Length (mm)</label>
                            <AdminInput v-model="lengthMm" type="number" step="0.01" placeholder="e.g. 2278" />
                            <p v-if="errors.length_mm" class="text-xs text-red-400 mt-1">{{ errors.length_mm }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Width (mm)</label>
                            <AdminInput v-model="widthMm" type="number" step="0.01" placeholder="e.g. 1134" />
                            <p v-if="errors.width_mm" class="text-xs text-red-400 mt-1">{{ errors.width_mm }}</p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Thickness (mm)</label>
                            <AdminInput v-model="thicknessMm" type="number" step="0.01" placeholder="e.g. 35" />
                            <p v-if="errors.thickness_mm" class="text-xs text-red-400 mt-1">{{ errors.thickness_mm }}
                            </p>
                        </div>

                        <div>
                            <label class="block text-sm font-semibold text-zinc-300 mb-2">Weight (kg)</label>
                            <AdminInput v-model="weightKg" type="number" step="0.01" placeholder="e.g. 27.5" />
                            <p v-if="errors.weight_kg" class="text-xs text-red-400 mt-1">{{ errors.weight_kg }}</p>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Cost Price</label>
                        <AdminInput v-model="costPrice" type="number" step="0.01" placeholder="e.g. 15000" />
                        <p v-if="errors.cost_price" class="text-xs text-red-400 mt-1">{{ errors.cost_price }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Compare At Cost Price</label>
                        <AdminInput v-model="compareAtCostPrice" type="number" step="0.01" placeholder="Optional" />
                        <p v-if="errors.compare_at_cost_price" class="text-xs text-red-400 mt-1">{{
                            errors.compare_at_cost_price }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Selling Price</label>
                        <AdminInput v-model="sellingPrice" type="number" step="0.01" placeholder="e.g. 19999" />
                        <p v-if="errors.selling_price" class="text-xs text-red-400 mt-1">{{ errors.selling_price }}</p>
                    </div>

                    <div>
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Compare At Selling Price</label>
                        <AdminInput v-model="compareAtSellingPrice" type="number" step="0.01" placeholder="Optional" />
                        <p v-if="errors.compare_at_selling_price" class="text-xs text-red-400 mt-1">{{
                            errors.compare_at_selling_price }}</p>
                    </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
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

                    <div class="sm:col-span-2">
                        <label class="block text-sm font-semibold text-zinc-300 mb-2">Image URL</label>
                        <AdminInput v-model="imageUrl" placeholder="https://..." />
                        <p v-if="errors.image_url" class="text-xs text-red-400 mt-1">{{ errors.image_url }}</p>
                    </div>
                </div>

                <label class="flex items-center gap-2 w-fit cursor-pointer select-none">
                    <input type="checkbox" v-model="isActive"
                        class="h-4 w-4 rounded border-zinc-600 bg-zinc-800 text-lime-main focus:ring-lime-main" />
                    <span class="text-sm font-semibold text-zinc-300">Active</span>
                </label>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Save Changes
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
const route = useRoute();
const productId = route.params.id;

const categories = ref([]);
const categoriesLoading = ref(true);

const subCategories = ref([]);
const subCategoriesLoading = ref(false);

const brands = ref([]);
const brandsLoading = ref(true);

const categoryId = ref(null);
const subCategoryId = ref(null);
const brandId = ref(null);
const productType = ref('other');

const name = ref('');
const slug = ref('');
const slugTouched = ref(true);
const model = ref('');
const description = ref('');
const content = ref('');

const powerRatingWatts = ref('');
const voltageRating = ref('');
const capacityAh = ref('');
const warrantyMonths = ref('');

const panelType = ref('');
const cellType = ref('');
const numberOfCells = ref('');
const efficiencyPercentage = ref('');
const maxSystemVoltage = ref('');
const openCircuitVoltage = ref('');
const shortCircuitCurrent = ref('');
const maxPowerVoltage = ref('');
const maxPowerCurrent = ref('');
const temperatureCoefficient = ref('');
const frameMaterial = ref('');
const glassType = ref('');
const lengthMm = ref('');
const widthMm = ref('');
const thicknessMm = ref('');
const weightKg = ref('');

const costPrice = ref('');
const compareAtCostPrice = ref('');
const sellingPrice = ref('');
const compareAtSellingPrice = ref('');

const quantityInStock = ref('0');
const reorderLevel = ref('0');
const unit = ref('piece');

const imageUrl = ref('');
const isActive = ref(true);

const errors = ref({});

const pageLoading = ref(true);

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const productTypeOptions = [
    { label: 'Other', value: 'other' },
    { label: 'Solar Panel', value: 'solar' }
];

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

watch(slug, (newValue) => {
    if (newValue !== slugify(name.value)) {
        slugTouched.value = true;
    }
});

watch(categoryId, async (newCategoryId, oldCategoryId) => {
    if (oldCategoryId !== undefined && newCategoryId !== oldCategoryId) {
        subCategoryId.value = null;
    }
    subCategories.value = [];
    if (!newCategoryId) return;
    await fetchSubCategories(newCategoryId);
});

watch(productType, (newType) => {
    if (newType !== 'solar') {
        panelType.value = '';
        cellType.value = '';
        numberOfCells.value = '';
        efficiencyPercentage.value = '';
        maxSystemVoltage.value = '';
        openCircuitVoltage.value = '';
        shortCircuitCurrent.value = '';
        maxPowerVoltage.value = '';
        maxPowerCurrent.value = '';
        temperatureCoefficient.value = '';
        frameMaterial.value = '';
        glassType.value = '';
        lengthMm.value = '';
        widthMm.value = '';
        thicknessMm.value = '';
        weightKg.value = '';
    }
});

function toNumberOrNull(value) {
    if (value === '' || value === null || value === undefined) return null;
    const n = Number(value);
    return Number.isNaN(n) ? null : n;
}

function toInputValue(value) {
    if (value === null || value === undefined) return '';
    return String(value);
}

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

async function fetchProduct() {
    pageLoading.value = true;
    try {
        const data = await authFetch('/api/admin/products/' + productId);
        if (data) {
            categoryId.value = data.category_id;
            subCategoryId.value = data.sub_category_id;
            brandId.value = data.brand_id;
            productType.value = data.product_type || 'other';

            name.value = data.name || '';
            slug.value = data.slug || '';
            model.value = data.model || '';
            description.value = data.description || '';
            content.value = data.content || '';

            powerRatingWatts.value = toInputValue(data.power_rating_watts);
            voltageRating.value = toInputValue(data.voltage_rating);
            capacityAh.value = toInputValue(data.capacity_ah);
            warrantyMonths.value = toInputValue(data.warranty_months);

            panelType.value = data.panel_type || '';
            cellType.value = data.cell_type || '';
            numberOfCells.value = toInputValue(data.number_of_cells);
            efficiencyPercentage.value = toInputValue(data.efficiency_percentage);
            maxSystemVoltage.value = toInputValue(data.max_system_voltage);
            openCircuitVoltage.value = toInputValue(data.open_circuit_voltage);
            shortCircuitCurrent.value = toInputValue(data.short_circuit_current);
            maxPowerVoltage.value = toInputValue(data.max_power_voltage);
            maxPowerCurrent.value = toInputValue(data.max_power_current);
            temperatureCoefficient.value = toInputValue(data.temperature_coefficient);
            frameMaterial.value = data.frame_material || '';
            glassType.value = data.glass_type || '';
            lengthMm.value = toInputValue(data.length_mm);
            widthMm.value = toInputValue(data.width_mm);
            thicknessMm.value = toInputValue(data.thickness_mm);
            weightKg.value = toInputValue(data.weight_kg);

            costPrice.value = toInputValue(data.cost_price);
            compareAtCostPrice.value = toInputValue(data.compare_at_cost_price);
            sellingPrice.value = toInputValue(data.selling_price);
            compareAtSellingPrice.value = toInputValue(data.compare_at_selling_price);

            quantityInStock.value = toInputValue(data.quantity_in_stock ?? 0);
            reorderLevel.value = toInputValue(data.reorder_level ?? 0);
            unit.value = data.unit || 'piece';

            imageUrl.value = data.image_url || '';
            isActive.value = !!data.is_active;

            if (data.sub_category_id) {
                await fetchSubCategories(data.category_id);
            }
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to load product.';
        showStatus.value = true;
    } finally {
        pageLoading.value = false;
    }
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

    if (compareAtCostPrice.value !== '' && Number(compareAtCostPrice.value) < 0) {
        errors.value.compare_at_cost_price = 'Compare at cost price must be 0 or more.';
    }

    if (compareAtSellingPrice.value !== '' && Number(compareAtSellingPrice.value) < 0) {
        errors.value.compare_at_selling_price = 'Compare at selling price must be 0 or more.';
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

    if (
        efficiencyPercentage.value !== '' &&
        (Number(efficiencyPercentage.value) < 0 || Number(efficiencyPercentage.value) > 100)
    ) {
        errors.value.efficiency_percentage = 'Efficiency must be between 0 and 100.';
    }

    return Object.keys(errors.value).length === 0;
}

async function handleSubmit() {
    if (!validate()) return;

    statusType.value = 'loading';
    statusMessage.value = 'Saving changes...';
    showStatus.value = true;

    try {
        const data = await authFetch('/api/admin/products/' + productId, {
            method: 'PATCH',
            body: {
                category_id: categoryId.value,
                sub_category_id: subCategoryId.value,
                brand_id: brandId.value,

                name: name.value.trim(),
                brand: null,
                model: model.value.trim() || null,
                description: description.value.trim() || null,
                content: content.value.trim() || null,
                product_type: productType.value,

                power_rating_watts: toNumberOrNull(powerRatingWatts.value),
                voltage_rating: toNumberOrNull(voltageRating.value),
                capacity_ah: toNumberOrNull(capacityAh.value),
                warranty_months: toNumberOrNull(warrantyMonths.value),

                panel_type: panelType.value.trim() || null,
                cell_type: cellType.value.trim() || null,
                number_of_cells: toNumberOrNull(numberOfCells.value),
                efficiency_percentage: toNumberOrNull(efficiencyPercentage.value),
                max_system_voltage: toNumberOrNull(maxSystemVoltage.value),
                open_circuit_voltage: toNumberOrNull(openCircuitVoltage.value),
                short_circuit_current: toNumberOrNull(shortCircuitCurrent.value),
                max_power_voltage: toNumberOrNull(maxPowerVoltage.value),
                max_power_current: toNumberOrNull(maxPowerCurrent.value),
                temperature_coefficient: toNumberOrNull(temperatureCoefficient.value),
                frame_material: frameMaterial.value.trim() || null,
                glass_type: glassType.value.trim() || null,
                length_mm: toNumberOrNull(lengthMm.value),
                width_mm: toNumberOrNull(widthMm.value),
                thickness_mm: toNumberOrNull(thicknessMm.value),
                weight_kg: toNumberOrNull(weightKg.value),

                cost_price: toNumberOrNull(costPrice.value),
                compare_at_cost_price: toNumberOrNull(compareAtCostPrice.value),
                selling_price: toNumberOrNull(sellingPrice.value),
                compare_at_selling_price: toNumberOrNull(compareAtSellingPrice.value),

                quantity_in_stock: toNumberOrNull(quantityInStock.value) ?? 0,
                reorder_level: toNumberOrNull(reorderLevel.value) ?? 0,
                unit: unit.value.trim(),

                image_url: imageUrl.value.trim() || null,
                is_active: isActive.value
            }
        });

        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Product updated.';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to update product.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}

await Promise.all([fetchCategories(), fetchBrands(), fetchProduct()]);
</script>