<template>
    <div class="w-full min-h-[80vh] flex items-center justify-center py-12">
        <div class="flex flex-col max-w-87.5 w-full">
            <div class="flex items-center m-auto mb-6">
                <img src="/logos/kaleem-solar-logo-t-2.webp" alt="Kaleem solar logo" width="190px">
            </div>

            <Loading v-if="verifying" message="Verifying your email..." />

            <div v-else-if="verified" class="flex flex-col">
                <AlertsSuccess v-if="message" :message="message" @close="message = ''" />
                <NuxtLink to="/login"
                    class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                    <span class="mr-3">Continue to login</span>
                    <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                        src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15"
                        alt="Arrow right icon">
                </NuxtLink>
            </div>

            <form v-else @submit.prevent="resendVerification" method="post">
                <AlertsError v-if="errors.message" :message="errors.message" />

                <p class="text-para-light text-sm mt-3 mb-3">
                    Enter your email address and we'll send you a new verification link.
                </p>

                <div class="grid grid-cols-1 gap-3">
                    <div class="flex flex-col">
                        <Input v-model="email" type="email" placeholder="Email address" />
                        <AlertsAlertError v-if="errors.email" :error="errors.email" />
                    </div>
                </div>

                <button v-if="!processing" type="submit"
                    class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                    <span class="mr-3">Resend verification link</span>
                    <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                        src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15"
                        alt="Arrow right icon">
                </button>

                <p class="text-para-light inline-block text-sm ml-1 mt-3">Already verified? <NuxtLink to="/login"
                        class="text-navy underline">Back to login</NuxtLink>
                </p>

                <div class="my-3 m-auto w-fit">
                    <NuxtTurnstile ref="turnstile" v-model="ct_token" />
                </div>

                <AlertsSuccess v-if="message" :message="message" @close="message = ''" />
                <Loading v-if="processing" message="Processing request..." />
            </form>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    layout: 'guest'
});

const route = useRoute();
const { authFetch } = useAuthFetch();

const token = typeof route.query.token === 'string' ? route.query.token.trim() : '';

const email = ref("");
const ct_token = ref("");
const turnstile = ref(null);
const verifying = ref(token !== '');
const verified = ref(false);
const processing = ref(false);
const message = ref("");
const errors = ref({
    count: 0
});

onMounted(() => {
    if (token) {
        verifyEmail();
    } else {
        errors.value.message = 'This verification link is invalid or incomplete.';
    }
});

async function verifyEmail() {
    verifying.value = true;
    reset_errors();
    try {
        const data = await authFetch('/api/account/verify-email', {
            method: "POST",
            body: { token }
        });
        verified.value = true;
        message.value = data?.message || 'Your email has been verified.';
    } catch (e) {
        errors.value.message = e.statusMessage || 'This link is invalid or has expired.';
    } finally {
        verifying.value = false;
    }
}

async function resendVerification() {
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
        const data = await authFetch('/api/account/resend-verification', {
            method: "POST",
            body: {
                email: email.value.trim(),
                ct_token: ct_token.value
            }
        });
        if (data) {
            message.value = data.message || 'If that email is registered, a new verification link has been sent.';
            email.value = "";
        }
    } catch (e) {
        errors.value.message = e.statusMessage || 'Something went wrong!';
    } finally {
        processing.value = false;
        turnstile.value?.reset();
    }
}

function reset_errors() {
    errors.value = {
        count: 0
    };
}
</script>