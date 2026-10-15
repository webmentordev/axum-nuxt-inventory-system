<template>
    <section class="h-full w-full p-6">
        <div class="max-w-lg">
            <h1 class="text-xl font-bold text-white">Create info card</h1>
            <p class="text-sm text-zinc-500 mt-1">Add a new info card.</p>

            <img src="/info-cards-banner.png" class="mt-1">

            <form @submit.prevent="handleSubmit" class="mt-3 flex flex-col gap-4" novalidate>
                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Offer title</label>
                    <AdminInput v-model="offerTitle" placeholder="e.g. Limited Offer, Weekend Sale" />
                    <p v-if="errors.offer_title" class="text-xs text-red-400 mt-1">{{ errors.offer_title }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Title</label>
                    <AdminInput v-model="title" placeholder="e.g. Laptop Sale - Unmissable Deals Await!" />
                    <p v-if="errors.title" class="text-xs text-red-400 mt-1">{{ errors.title }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Description</label>
                    <AdminTextarea v-model="description" placeholder="E.g Unleash Smart Savings Today!" rows="4" />
                    <p v-if="errors.description" class="text-xs text-red-400 mt-1">{{ errors.description }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Transparent item image URL</label>
                    <AdminInput v-model="imageUrl" placeholder="https://..." />
                    <p v-if="errors.image_url" class="text-xs text-red-400 mt-1">{{ errors.image_url }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Path URL</label>
                    <AdminInput v-model="pathUrl" placeholder="/products" />
                    <p v-if="errors.path_url" class="text-xs text-red-400 mt-1">{{ errors.path_url }}</p>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-zinc-300 mb-2">Sort order</label>
                    <AdminInput v-model="sortOrder" type="number" placeholder="0" />
                    <p v-if="errors.sort_order" class="text-xs text-red-400 mt-1">{{ errors.sort_order }}</p>
                </div>

                <label class="flex items-center gap-2 text-sm font-semibold text-zinc-300 cursor-pointer w-fit">
                    <input v-model="isActive" type="checkbox" class="accent-lime-500" />
                    Active
                </label>

                <button type="submit"
                    class="mt-2 px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors w-fit">
                    Create info card
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

const offerTitle = ref('');
const title = ref('');
const description = ref('');
const imageUrl = ref('');
const pathUrl = ref('');
const sortOrder = ref(0);
const isActive = ref(true);
const errors = ref({});

const showStatus = ref(false);
const statusType = ref('loading');
const statusMessage = ref('');

async function handleSubmit() {
    if (!validate()) return;
    statusType.value = 'loading';
    statusMessage.value = 'Creating info card...';
    showStatus.value = true;
    try {
        const data = await authFetch('/api/admin/info-cards', {
            method: 'POST',
            body: {
                offer_title: offerTitle.value.trim(),
                title: title.value.trim(),
                description: description.value.trim(),
                image_url: imageUrl.value.trim() || null,
                path_url: pathUrl.value.trim(),
                sort_order: Number(sortOrder.value) || 0,
                is_active: isActive.value
            }
        });
        if (data) {
            statusType.value = 'success';
            statusMessage.value = 'Info card created.';
            offerTitle.value = '';
            title.value = '';
            description.value = '';
            imageUrl.value = '';
            pathUrl.value = '';
            sortOrder.value = 0;
            isActive.value = true;
        }
    } catch (e) {
        statusType.value = 'error';
        statusMessage.value = e.statusMessage || 'Failed to create info card.';
    } finally {
        setTimeout(() => {
            showStatus.value = false;
        }, 5000)
    }
};

function validate() {
    errors.value = {};
    if (!offerTitle.value.trim()) {
        errors.value.offer_title = 'Offer title is required.';
    } else if (offerTitle.value.trim().length > 150) {
        errors.value.offer_title = 'Offer title must be under 150 characters.';
    }
    if (!title.value.trim()) {
        errors.value.title = 'Title is required.';
    } else if (title.value.trim().length < 2) {
        errors.value.title = 'Title must be at least 2 characters.';
    } else if (title.value.trim().length > 150) {
        errors.value.title = 'Title must be under 150 characters.';
    }
    if (!description.value.trim()) {
        errors.value.description = 'Description is required.';
    } else if (description.value.trim().length > 500) {
        errors.value.description = 'Description must be under 500 characters.';
    }
    if (imageUrl.value.trim().length > 500) {
        errors.value.image_url = 'Image URL must be under 500 characters.';
    }
    if (!pathUrl.value.trim()) {
        errors.value.path_url = 'Path URL is required.';
    } else if (pathUrl.value.trim().length > 500) {
        errors.value.path_url = 'Path URL must be under 500 characters.';
    }
    if (Number(sortOrder.value) < 0 || !Number.isInteger(Number(sortOrder.value))) {
        errors.value.sort_order = 'Sort order must be a non-negative integer.';
    }
    return Object.keys(errors.value).length === 0;
}
</script>