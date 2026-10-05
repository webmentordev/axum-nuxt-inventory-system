<template>
    <div class="w-full min-h-[80vh] flex items-center justify-center py-12">
        <div class="flex flex-col max-w-87.5 w-full">
            <div class="flex items-center m-auto mb-6">
                <img src="/logos/kaleem-solar-logo-t-2.webp" alt="Kaleem solar logo" width="190px">
            </div>
            <form @submit.prevent="resetPassword" method="post">
                <div class="grid grid-cols-1 gap-3">
                    <div class="flex flex-col">
                        <Input v-model="password" type="password" placeholder="New password" />
                        <AlertsAlertError v-if="errors.password" :error="errors.password" />
                    </div>
                    <div class="flex flex-col">
                        <Input v-model="confirmPassword" type="password" placeholder="Confirm new password" />
                        <AlertsAlertError v-if="errors.confirmPassword" :error="errors.confirmPassword" />
                    </div>
                </div>
                <button v-if="!processing" type="submit"
                    class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                    <span class="mr-3">Reset password</span>
                    <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                        src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15"
                        alt="Arrow right icon">
                </button>

                <p class="text-para-light inline-block text-sm ml-1 mt-3">Back to <NuxtLink to="/login"
                        class="text-navy underline">Login</NuxtLink>
                </p>

                <div class="my-3 m-auto w-fit">
                    <NuxtTurnstile ref="turnstile" v-model="ct_token" />
                </div>

                <AlertsSuccess v-if="message" :message="message" @close="message = ''" />
                <Loading v-if="processing" message="Processing request..." />
                <AlertsError v-if="errors.message" :message="errors.message" />
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
const route = useRoute();

const token = computed(() => route.query.token || "");
const password = ref("");
const confirmPassword = ref("");
const ct_token = ref("");
const processing = ref(false);
const message = ref("");
const errors = ref({
    count: 0
});

async function resetPassword() {
    processing.value = true;
    message.value = "";
    reset_errors();
    if (!token.value) {
        errors.value.message = "Invalid or missing reset token";
        errors.value.count += 1;
    }
    if (password.value.trim() == "") {
        errors.value.password = "Password is required";
        errors.value.count += 1;
    } else if (password.value.trim().length < 8) {
        errors.value.password = "Password must be at least 8 characters";
        errors.value.count += 1;
    }
    if (confirmPassword.value.trim() == "") {
        errors.value.confirmPassword = "Please confirm your password";
        errors.value.count += 1;
    } else if (password.value.trim() != confirmPassword.value.trim()) {
        errors.value.confirmPassword = "Passwords do not match";
        errors.value.count += 1;
    }
    if (errors.value.count > 0) {
        processing.value = false;
        return;
    }
    try {
        const data = await authFetch('/api/account/reset-password', {
            method: "POST",
            body: {
                token: token.value,
                password: password.value.trim(),
                ct_token: ct_token.value
            }
        });
        if (data) {
            message.value = data.message || 'Password has been reset successfully.';
            password.value = "";
            confirmPassword.value = "";
            setTimeout(() => navigateTo('/login'), 2000);
        }
    } catch (e) {
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