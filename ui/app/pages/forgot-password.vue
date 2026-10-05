<template>
    <div class="w-full min-h-[80vh] flex items-center justify-center py-12">
        <div class="flex flex-col max-w-87.5 w-full">
            <div class="flex items-center m-auto mb-6">
                <img src="/logos/kaleem-solar-logo-t-2.webp" alt="Kaleem solar logo" width="190px">
            </div>
            <form @submit.prevent="forgotPassword" method="post">
                <div class="grid grid-cols-1 gap-3">
                    <div class="flex flex-col">
                        <Input v-model="email" type="email" placeholder="Email address" />
                        <AlertsAlertError v-if="errors.email" error="Email field is required" />
                    </div>
                </div>
                <button v-if="!processing" type="submit"
                    class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                    <span class="mr-3">Send reset link</span>
                    <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                        src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15"
                        alt="Arrow right icon">
                </button>

                <p class="text-para-light inline-block text-sm ml-1 mt-3">Remembered your password? <NuxtLink
                        to="/login" class="text-navy underline">Back to login</NuxtLink>
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

const email = ref("");
const ct_token = ref("");
const processing = ref(false);
const message = ref("");
const errors = ref({
    count: 0
});

async function forgotPassword() {
    processing.value = true;
    message.value = "";
    reset_errors();
    if (email.value.trim() == "") {
        errors.value.email = "Email is required";
        errors.value.count += 1;
    }
    if (errors.value.count > 0) {
        processing.value = false;
        return;
    }
    try {
        const data = await authFetch('/api/account/forgot-password', {
            method: "POST",
            body: {
                email: email.value.trim(),
                ct_token: ct_token.value
            }
        });
        if (data) {
            message.value = data.message || 'Password reset link has been sent to your email.';
            email.value = "";
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