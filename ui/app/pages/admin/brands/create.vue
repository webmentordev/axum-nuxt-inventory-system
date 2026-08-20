<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Create brand</h1>
            <p class="text-sm text-zinc-500 mt-1">Add a new brand for your products.</p>

            <form @submit.prevent="handleSubmit" class="mt-6 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Name</label>
                    <AdminInput v-model="name" placeholder="e.g. Electronics" />
                    <p v-if="errors.name" class="text-xs text-red-400 mt-1">{{ errors.name }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Description</label>
                    <AdminTextarea v-model="description" placeholder="Optional description..." rows="4" />
                    <p v-if="errors.description" class="text-xs text-red-400 mt-1">{{ errors.description }}</p>
                </div>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Create brand
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

const name = ref('');
const description = ref('');
const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

async function handleSubmit() {
    if (!validate()) return;
    statusType.value = 'loading';
    statusMessage.value = 'Creating brand...';
    showStatus.value = true;
    try {
        const data = await authFetch('/api/brands', {
            method: 'POST',
            body: {
                name: name.value.trim(),
                description: description.value.trim()
            }
        });
        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Brand created.';
            name.value = '';
            description.value = '';
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to create brand.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000)
    }
};

function validate() {
    errors.value = {};
    if (!name.value.trim()) {
        errors.value.name = 'Name is required.';
    } else if (name.value.trim().length < 2) {
        errors.value.name = 'Name must be at least 2 characters.';
    }
    if (!description.value.trim()) {
        errors.value.description = 'Description is required.';
    } else if (description.value.trim().length > 500) {
        errors.value.description = 'Description must be under 500 characters.';
    }
    return Object.keys(errors.value).length === 0;
}
</script>