<template>
    <div class="w-full min-h-[80vh] flex items-center justify-center py-12">
        <div class="flex flex-col max-w-87.5 w-full">
            <div class="flex items-center m-auto mb-6">
                <img src="/logos/kaleem-solar-logo-t-2.webp" alt="Kaleem solar logo" width="190px">
            </div>
            <form @submit.prevent="login" method="post">
                <div class="grid grid-cols-1 gap-3">
                    <div class="flex flex-col">
                        <Input v-model="email" type="email" placeholder="Email address" />
                        <AlertsAlertError v-if="errors.email" :error="errors.email" />
                    </div>
                    <div class="flex flex-col">
                        <Input v-model="password" type="password" placeholder="Password" />
                        <AlertsAlertError v-if="errors.password" :error="errors.password" />
                    </div>
                </div>
                <button v-if="!processing" type="submit"
                    class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                    <span class="mr-3">Login</span>
                    <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                        src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15"
                        alt="Arrow right icon">
                </button>

                <p class="text-para-light inline-block text-sm ml-1 mt-3">
                    <NuxtLink to="/forgot-password" class="text-navy underline">Forgot password?</NuxtLink>
                </p>

                <p class="text-para-light inline-block text-sm ml-1 mt-3">Don't have an account? <NuxtLink
                        to="/register" class="text-navy underline">Register here</NuxtLink>
                </p>

                <AlertsSuccess v-if="message" :message="message" @close="message = ''" />
                <Loading v-if="processing" message="Processing request..." />
                <AlertsError v-if="errors.message" :message="errors.message" />

                <p v-if="needsVerification" class="text-para-light text-sm ml-1 mt-2">
                    Didn't get the email? <NuxtLink to="/verify-email" class="text-navy underline">Resend
                        verification link</NuxtLink>
                </p>
            </form>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'guest',
    layout: 'guest'
});
const { authFetch } = useAuthFetch();
const { setToken } = useAuthToken();
const { setUser } = useAuthUser();

const email = ref("");
const password = ref("");
const processing = ref(false);
const message = ref("");
const needsVerification = ref(false);
const errors = ref({
    count: 0
});

async function login() {
    processing.value = true;
    message.value = "";
    needsVerification.value = false;
    reset_errors();
    if (email.value.trim() == "") {
        errors.value.email = "Email is required";
        errors.value.count += 1;
    }
    if (password.value == "") {
        errors.value.password = "Password is required";
        errors.value.count += 1;
    }
    if (errors.value.count > 0) {
        processing.value = false;
        return;
    }
    try {
        const data = await authFetch('/api/account/login', {
            method: "POST",
            body: {
                email: email.value.trim(),
                password: password.value
            }
        });
        if (data) {
            setToken(data.token);
            setUser(data.user);
            if (data.user.is_admin == true) {
                await navigateTo('/admin/dashboard');
            } else {
                await navigateTo('/');
            }
        }
    } catch (e) {
        const code = e.data?.data?.code || e.data?.code;
        if (code === 'email_not_verified') {
            needsVerification.value = true;
        }
        errors.value.message = e.statusMessage || 'Something went wrong!';
    } finally {
        processing.value = false;
    }
}

function reset_errors() {
    errors.value = {
        count: 0
    };
}
</script>