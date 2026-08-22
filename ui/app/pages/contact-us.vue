<template>
    <div class="w-full min-h-[80vh] flex items-center justify-center">
        <div class="w-full max-w-5xl mx-auto px-4 py-16 grid grid-cols-1 md:grid-cols-2 gap-10">
            <div class="flex flex-col">
                <div class="flex items-center mb-6">
                    <img src="/kaleem-solar-logo-t-2.png" alt="Kaleem solat logo" width="190px">
                </div>

                <h1 class="text-2xl font-bold text-gray-800 mb-1">Get in touch</h1>
                <p class="text-sm text-gray-500 mb-6">Have a question about our products? Send us a message.</p>

                <form @submit.prevent="submit" method="post">
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
                            <Input v-model="subject" type="text" placeholder="Subject" />
                            <AlertsAlertError v-if="errors.subject" error="Subject field is required" />
                        </div>
                        <div class="flex flex-col">
                            <Textarea v-model="message" placeholder="Your message" rows="5" />
                            <AlertsAlertError v-if="errors.message_field" error="Message field is required" />
                        </div>
                    </div>

                    <div class="my-3">
                        <NuxtTurnstile ref="turnstile" v-model="ct_token" />
                    </div>

                    <button v-if="!processing" type="submit"
                        class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                        <span class="mr-3">Send message</span>
                        <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                            src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15">
                    </button>

                    <AlertsSuccess v-if="successMessage" :message="successMessage" @close="successMessage = ''" />
                    <Loading v-if="processing" message="Sending your message..." />
                    <AlertsError v-if="errors.message" :message="errors.message" />
                </form>
            </div>

            <div class="flex flex-col">
                <div class="rounded-xl overflow-hidden border border-gray-200 mb-6 h-56 w-full">
                    <iframe class="w-full h-full border-0" loading="lazy" referrerpolicy="no-referrer-when-downgrade"
                        src="https://www.google.com/maps?q=Multan+Cantt,+Multan,+Punjab,+Pakistan&output=embed">
                    </iframe>
                </div>

                <div class="flex flex-col gap-5">
                    <div class="flex items-start gap-3">
                        <img src="https://api.iconify.design/mdi-light:phone.svg?color=%23062B5B" width="20"
                            class="mt-0.5">
                        <div>
                            <p class="text-sm font-semibold text-gray-800">Phone</p>
                            <p class="text-sm text-gray-500">+92 300 1234567</p>
                        </div>
                    </div>

                    <div class="flex items-start gap-3">
                        <img src="https://api.iconify.design/mdi-light:map-marker.svg?color=%23062B5B" width="20"
                            class="mt-0.5">
                        <div>
                            <p class="text-sm font-semibold text-gray-800">Address</p>
                            <p class="text-sm text-gray-500">Shop # 12, Cantt Bazaar, Multan Cantt, Multan, Punjab,
                                Pakistan</p>
                        </div>
                    </div>

                    <div class="flex items-start gap-3">
                        <img src="https://api.iconify.design/mdi-light:clock.svg?color=%23062B5B" width="20"
                            class="mt-0.5">
                        <div>
                            <p class="text-sm font-semibold text-gray-800">Open hours</p>
                            <p class="text-sm text-gray-500">Mon – Sat: 9:00 AM – 8:00 PM</p>
                            <p class="text-sm text-gray-500">Sunday: Closed</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    layout: 'public'
});
const { publicFetch } = usePublicFetch();

const turnstile = ref();
const name = ref("");
const email = ref("");
const subject = ref("");
const message = ref("");
const ct_token = ref("");
const processing = ref(false);
const successMessage = ref(false);
const errors = ref({
    count: 0
});

async function submit() {
    processing.value = true;
    successMessage.value = false;
    reset_errors();
    if (name.value == "") {
        errors.value.name = "Name is required";
        errors.value.count += 1;
    }
    if (email.value == "") {
        errors.value.email = "Email is required";
        errors.value.count += 1;
    }
    if (subject.value == "") {
        errors.value.subject = "Subject is required";
        errors.value.count += 1;
    }
    if (message.value == "") {
        errors.value.message_field = "Message is required";
        errors.value.count += 1;
    }
    if (errors.value.count > 0) {
        processing.value = false;
        return;
    };
    try {
        const data = await publicFetch('/api/public/contacts', {
            method: "POST",
            body: {
                name: name.value.trim(),
                email: email.value.trim(),
                subject: subject.value.trim(),
                message: message.value.trim(),
                ct_token: ct_token.value
            }
        });
        if (data) {
            successMessage.value = "Your message has been sent!";
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
    subject.value = "";
    message.value = "";
    turnstile.value?.reset();
}
</script>