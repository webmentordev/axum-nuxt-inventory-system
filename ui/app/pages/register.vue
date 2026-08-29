<template>
    <div class="w-full min-h-[80vh]">
        <div class="w-full h-full flex items-center justify-center">
            <div class="flex flex-col max-w-87.5 w-full">
                <div class="flex items-center m-auto mb-6">
                    <img src="/kaleem-solar-logo-t-2.png" alt="Kaleem solat logo" width="190px">
                </div>
                <form @submit.prevent="login" method="post">
                    <div class="grid grid-cols-1 gap-3">
                        <div class="flex flex-col">
                            <Input v-model="name" type="text" placeholder="Name" />
                            <AlertsAlertError v-if="errors.name" error="Name field is required" />
                        </div>
                        <div class="flex flex-col">
                            <Input v-model="email" type="email" placeholder="Email address" />
                            <AlertsAlertError v-if="errors.email" error="Email field is required" />
                        </div>
                        <div class="flex flex-col">
                            <Input v-model="password" type="password" placeholder="Password" />
                            <AlertsAlertError v-if="errors.password" error="Password field is required" />
                        </div>
                        <div class="flex flex-col">
                            <Input v-model="confirm_password" type="password" placeholder="Confirm password" />
                            <AlertsAlertError v-if="errors.confirm_password"
                                error="Confirm password field is required" />
                        </div>
                    </div>

                    <div class="my-3">
                        <NuxtTurnstile ref="turnstile" v-model="ct_token" />
                    </div>

                    <button v-if="!processing" type="submit"
                        class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                        <span class="mr-3">Register</span>
                        <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                            src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15">
                    </button>


                    <p class="text-para-light inline-block text-sm ml-1 mt-3">Already have an account? <NuxtLink
                            to="/login" class="text-navy underline">Login here</NuxtLink>
                    </p>

                    <p class="text-para-light inline-block text-sm ml-1 mt-3 pt-3 border-t border-gray-200">By
                        continuing, you agree to our <NuxtLink to="/terms-of-service" class="text-navy underline">Terms
                            of service</NuxtLink> & <NuxtLink to="/privacy-policy" class="text-navy underline">Privacy
                            policy</NuxtLink>
                    </p>

                    <AlertsSuccess v-if="message" :message="message" @close="message = ''" />
                    <Loading v-if="processing" message="Processing signup request..." />
                    <AlertsError v-if="errors.message" :message="errors.message" />
                </form>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'guest',
    layout: 'guest'
});
const { authFetch } = useAuthFetch();

const name = ref("");
const email = ref("");
const password = ref("");
const confirm_password = ref("");
const processing = ref(false);
const message = ref(false);
const ct_token = ref("");
const errors = ref({
    count: 0
});

async function login() {
    processing.value = true;
    message.value = false;
    reset_errors();
    if (name.value == "") {
        errors.value.name = "Name is required";
        errors.value.count += 1;
    }
    if (email.value == "") {
        errors.value.email = "Email is required";
        errors.value.count += 1;
    }
    if (password.value == "") {
        errors.value.password = "Password is required";
        errors.value.count += 1;
    }
    if (confirm_password.value == "") {
        errors.value.confirm_password = "Password confirmation is required";
        errors.value.count += 1;
    }
    if (password.value != confirm_password.value) {
        errors.value.password = "Password and Confirm password is required";
        errors.value.count += 1;
    }
    if (errors.value.count > 0) {
        processing.value = false;
        return;
    };
    try {
        const data = await authFetch('/api/account/register', {
            method: "POST",
            body: {
                name: name.value.trim(),
                email: email.value.trim(),
                password: password.value.trim(),
                confirm_password: confirm_password.value.trim(),
                ct_token: ct_token.value
            }
        });
        if (data) {
            message.value = "Account has been created!";
            reset_values();
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

function reset_values() {
    name.value = "";
    email.value = "";
    password.value = "";
    confirm_password.value = "";
}

</script>