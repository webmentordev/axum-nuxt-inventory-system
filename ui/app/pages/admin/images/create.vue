<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Upload Image</h1>
            <p class="text-sm text-zinc-500 mt-1">Attach an image to a category, sub-category, or brand.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Attach to <span
                            class="text-zinc-500 font-normal">(optional)</span></label>
                    <AdminSelect v-model="targetType" :options="targetTypeOptions" placeholder="None"
                        @update:modelValue="handleTargetTypeChange" />
                </div>

                <div v-if="targetType">
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">{{ targetLabel }}</label>
                    <AdminSelect v-model="targetId" :options="targetOptions"
                        :placeholder="targetLoading ? `Loading ${targetLabel.toLowerCase()}...` : `Select a ${targetLabel.toLowerCase()}`" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Name</label>
                    <AdminInput v-model="name" placeholder="e.g. Jinko logo" />
                    <p v-if="errors.name" class="text-xs text-red-400 mt-1">{{ errors.name }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Image</label>

                    <label
                        class="flex flex-col items-center justify-center w-full border border-dashed border-dark-300 bg-dark-200 rounded-xl p-6 cursor-pointer hover:border-lime-main/50 transition-colors">
                        <img v-if="previewUrl" :src="previewUrl" alt="Preview"
                            class="w-24 h-24 object-cover rounded-lg mb-3 border border-dark-300" />
                        <Icon v-else name="mdi-light:cloud-upload" size="32" class="text-zinc-500 mb-2" />
                        <span class="text-sm text-zinc-400">
                            {{ image ? image.name : 'Click to select an image' }}
                        </span>
                        <input type="file" accept="image/*" class="hidden" @change="handleFileChange" />
                    </label>
                    <p v-if="errors.image" class="text-xs text-red-400 mt-1">{{ errors.image }}</p>
                </div>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Upload Image
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

const targetTypeOptions = [
    { label: 'Product', value: 'product_id' },
    { label: 'Category', value: 'category_id' },
    { label: 'Sub-category', value: 'sub_category_id' },
    { label: 'Brand', value: 'brand_id' }
];

const targetType = ref(null);
const targetId = ref(null);
const targetOptionsMap = ref({});
const targetLoading = ref(false);

const name = ref('');
const image = ref(null);
const previewUrl = ref('');
const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

const targetLabel = computed(() => {
    const match = targetTypeOptions.find((option) => option.value === targetType.value);
    return match ? match.label : '';
});

const targetEndpoints = {
    category_id: '/api/categories',
    sub_category_id: '/api/sub-categories',
    brand_id: '/api/brands',
    product_id: '/api/products/list',
};

const targetOptions = computed(() => targetOptionsMap.value[targetType.value] || []);

async function handleTargetTypeChange(value) {
    targetId.value = null;
    if (!value || targetOptionsMap.value[value]) return;

    targetLoading.value = true;
    try {
        const data = await authFetch(targetEndpoints[value]);
        if (data) {
            targetOptionsMap.value[value] = data.map((item) => ({
                label: item.name,
                value: item.id
            }));
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Failed to load options.';
    } finally {
        targetLoading.value = false;
    }
}

function handleFileChange(event) {
    const file = event.target.files[0];
    if (!file) return;
    image.value = file;
    previewUrl.value = URL.createObjectURL(file);
}

function validate() {
    errors.value = {};

    if (!name.value.trim()) {
        errors.value.name = 'Name is required.';
    }
    if (!image.value) {
        errors.value.image = 'Please select an image.';
    }

    return Object.keys(errors.value).length === 0;
}

async function handleSubmit() {
    if (!validate()) return;

    statusType.value = 'loading';
    statusMessage.value = 'Uploading image...';
    showStatus.value = true;

    try {
        const formData = new FormData();
        formData.append('name', name.value.trim());
        formData.append('file', image.value);
        if (targetType.value && targetId.value) {
            formData.append(targetType.value, targetId.value);
        }

        const data = await authFetch('/api/images', {
            method: 'POST',
            body: formData
        });

        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Image uploaded.';
            targetType.value = null;
            targetId.value = null;
            name.value = '';
            image.value = null;
            previewUrl.value = '';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to upload image.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}
</script>