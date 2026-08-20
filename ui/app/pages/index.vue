<template>
    <div class="max-w-2xl m-auto w-full p-3">
        <h1>Welcome to Kaleem solar e-commerce store</h1>
        <Button @click="logout" class="w-fit mb-3" text="Logout" />
        <br>
        <Button @click="get_products" class="w-fit mb-3" text="Get products" />
        <Button @click="get_categories" class="w-fit mb-3" text="Get categories" />
        <br>
        <Input type="file" class="mb-3" required @change="handleFileChange" />
        <Button @click="create_image" class="w-fit mb-3" text="Upload image" />
        <Input v-model="image_id" type="text" class="mb-3" placeholder="ImageID" />
        <Button @click="delete_image" class="w-fit mb-3" text="Delete images" />
        <Button @click="get_images" class="w-fit mb-3" text="Get images" />
        <br>
        <Input v-model="contact_id" type="text" class="mb-3" placeholder="ContactID" />
        <Button @click="delete_contact" class="w-fit mb-3" text="Delete contact" />
        <Button @click="get_contacts" class="w-fit mb-3" text="Get contact" />
        <Button @click="create_contact" class="w-fit mb-3" text="Create contact" />
    </div>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth',
    layout: 'public'
});
const { removeToken } = useAuthToken();
const { authFetch } = useAuthFetch();
const { publicFetch } = usePublicFetch();

const image = ref(null);
const image_id = ref("");
const contact_id = ref("");

async function logout() {
    removeToken();
    await navigateTo('/login');
}

async function get_products() {
    try {
        const data = await authFetch('/api/products');
        if (data) {
            console.log(data);
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

async function get_images() {
    try {
        const data = await authFetch('/api/images');
        if (data) {
            console.log(data);
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

const handleFileChange = (event) => {
    const file = event.target.files[0]
    if (!file) return
    image.value = file
}

async function create_image() {
    if (!image.value) return;
    try {
        const formData = new FormData()
        formData.append('name', 'test-file')
        formData.append('file', image.value)
        const data = await authFetch('/api/images', {
            method: "POST",
            body: formData
        });
        if (data) {
            console.log(data);
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

async function delete_image() {
    try {
        const data = await authFetch('/api/images/' + image_id.value, {
            method: "DELETE",
        });
        if (data) {
            console.log(data);
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

async function get_contacts() {
    try {
        const data = await authFetch('/api/contacts');
        if (data) {
            console.log(data);
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}

async function create_contact() {
    try {
        const data = await publicFetch('/api/contacts', {
            method: "POST",
            body: {
                "name": "Ahmer",
                "email": "ahemr@ystsol.com",
                "subject": "Unknown item",
                "message": "I would like to buy it!"
            }
        });
        if (data) {
            console.log(data);
        }
    } catch (e) {
        console.log(e);
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}


async function delete_contact() {
    try {
        const data = await authFetch('/api/contacts/' + contact_id.value, {
            method: "DELETE",
        });
        if (data) {
            console.log(data);
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    }
}
</script>