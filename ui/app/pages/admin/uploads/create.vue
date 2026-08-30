<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Upload File</h1>
            <p class="text-sm text-zinc-500 mt-1">Attach a file to a category, sub-category, or brand.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">File Type</label>
                    <AdminSelect v-model="fileType" :options="fileTypeOptions" />
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Assignment</label>
                    <AdminSelect v-model="assignMode" :options="assignModeOptions"
                        @update:modelValue="handleAssignModeChange" />
                </div>

                <div v-if="assignMode === 'assign'">
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Attach to</label>
                    <AdminSelect v-model="targetType" :options="targetTypeOptions" placeholder="Select a type"
                        @update:modelValue="handleTargetTypeChange" />
                </div>

                <div v-if="assignMode === 'assign' && targetType">
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
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">File</label>

                    <label
                        class="flex flex-col items-center justify-center w-full border border-dashed border-dark-300 bg-dark-200 rounded-xl p-6 cursor-pointer hover:border-lime-main/50 transition-colors">
                        <img v-if="previewUrl" :src="previewUrl" alt="Preview"
                            class="w-24 h-24 object-cover rounded-lg mb-3 border border-dark-300" />
                        <Icon v-else name="mdi-light:cloud-upload" size="32" class="text-zinc-500 mb-2" />
                        <span class="text-sm text-zinc-400">
                            {{ file ? file.name : 'Click to select a file' }}
                        </span>
                        <input type="file" class="hidden" @change="handleFileChange" />
                    </label>
                    <p v-if="errors.file" class="text-xs text-red-400 mt-1">{{ errors.file }}</p>
                </div>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Upload File
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

const fileTypeOptions = [
    { label: 'Image', value: 'image' },
    { label: 'File', value: 'file' }
];

const assignModeOptions = [
    { label: "Don't assign", value: 'none' },
    { label: 'Assign', value: 'assign' }
];

const targetTypeOptions = [
    { label: 'Product', value: 'product_id' },
    { label: 'Category', value: 'category_id' },
    { label: 'Sub-category', value: 'sub_category_id' },
    { label: 'Brand', value: 'brand_id' }
];

const fileType = ref('image');
const assignMode = ref('none');
const targetType = ref(null);
const targetId = ref(null);
const targetOptionsMap = ref({});
const targetLoading = ref(false);

const name = ref('');
const file = ref(null);
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
    category_id: '/api/admin/categories',
    sub_category_id: '/api/admin/sub-categories',
    brand_id: '/api/admin/brands',
    product_id: '/api/admin/products/list',
};

const targetOptions = computed(() => targetOptionsMap.value[targetType.value] || []);

function handleAssignModeChange(value) {
    if (value !== 'assign') {
        targetType.value = null;
        targetId.value = null;
    }
}

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
    const selected = event.target.files[0];
    if (!selected) return;
    file.value = selected;
    if (fileType.value === 'image') {
        previewUrl.value = URL.createObjectURL(selected);
    } else {
        previewUrl.value = '';
    }
}

function validate() {
    errors.value = {};

    if (!name.value.trim()) {
        errors.value.name = 'Name is required.';
    }
    if (!file.value) {
        errors.value.file = 'Please select a file.';
    }

    return Object.keys(errors.value).length === 0;
}

async function handleSubmit() {
    if (!validate()) return;

    statusType.value = 'loading';
    statusMessage.value = 'Uploading file...';
    showStatus.value = true;

    try {
        const formData = new FormData();
        formData.append('name', name.value.trim());
        formData.append('file_type', fileType.value);
        formData.append('file', file.value);
        if (assignMode.value === 'assign' && targetType.value && targetId.value) {
            formData.append(targetType.value, targetId.value);
        }

        const data = await authFetch('/api/admin/uploads', {
            method: 'POST',
            body: formData
        });

        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'File uploaded.';
            assignMode.value = 'none';
            targetType.value = null;
            targetId.value = null;
            fileType.value = 'image';
            name.value = '';
            file.value = null;
            previewUrl.value = '';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to upload file.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000);
    }
}
</script>