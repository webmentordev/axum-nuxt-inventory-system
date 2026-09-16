<template>
    <div class="w-full min-h-[80vh] flex items-center justify-center py-12">
        <div class="flex flex-col max-w-87.5 w-full">
            <div class="flex items-center m-auto mb-6">
                <img src="/logos/kaleem-solar-logo-t-2.webp" alt="Kaleem solar logo" width="190px">
            </div>

            <form v-if="!result" @submit.prevent="verifyBarcode" method="post">
                <div class="grid grid-cols-1 gap-3">
                    <div class="flex flex-col">
                        <Input v-model="barcode" type="text" placeholder="Barcode" />
                        <AlertsAlertError v-if="errors.barcode" error="Barcode is required" />
                    </div>
                </div>
                <button v-if="!processing" type="submit"
                    class="bg-navy mt-4 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group">
                    <span class="mr-3">Verify Barcode</span>
                    <img class="mt-1 transition-all group-hover:transition-all group-hover:translate-x-4"
                        src="https://api.iconify.design/line-md:arrow-right.svg?color=%23ffffff" width="15">
                </button>

                <p class="text-para-light inline-block text-sm ml-1 mt-3">Need help? <NuxtLink to="/contact-us"
                        class="text-navy underline">Contact us</NuxtLink>
                </p>

                <Loading v-if="processing" message="Verifying barcode..." />
                <AlertsError v-if="errors.message" :message="errors.message" />

                <div class="my-3 flex items-center justify-center">
                    <NuxtTurnstile ref="turnstile" v-model="ct_token" />
                </div>
            </form>

            <div v-else-if="result.exists" class="flex flex-col gap-4">
                <div class="flex flex-col items-center text-center gap-2 mb-2">
                    <img class="w-14 h-14" src="https://api.iconify.design/line-md:confirm-circle.svg?color=%2316a34a"
                        width="56">
                    <h1 class="text-xl font-semibold">Genuine Product</h1>
                    <p class="text-sm text-para-light">This item was purchased from us.</p>
                </div>

                <div v-if="result.product_name" class="border-t border-gray-200 pt-4">
                    <p class="font-medium mb-2">Product</p>
                    <p class="text-sm">{{ result.product_name }}</p>
                </div>

                <div v-if="result.purchased_at" class="border-t border-gray-200 pt-4">
                    <p class="font-medium mb-2">Purchase Date</p>
                    <p class="text-sm">{{ formatDate(result.purchased_at) }}</p>
                </div>

                <button @click="reset"
                    class="bg-navy mt-2 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90">
                    Verify another barcode
                </button>
            </div>

            <div v-else class="flex flex-col gap-4">
                <div class="flex flex-col items-center text-center gap-2 mb-2">
                    <img class="w-14 h-14" src="https://api.iconify.design/line-md:close-circle.svg?color=%23dc2626"
                        width="56">
                    <h1 class="text-xl font-semibold">Not Verified</h1>
                    <p class="text-sm text-para-light">We couldn't confirm this item was purchased from us.</p>
                </div>

                <button @click="reset"
                    class="bg-navy mt-2 text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90">
                    Try another barcode
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    layout: 'guest'
});
const { publicFetch } = usePublicFetch();

const barcode = ref('')
const processing = ref(false)
const result = ref(null)
const errors = reactive({ barcode: false, message: '' })
const ct_token = ref("");

const formatDate = (value) => new Date(value).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric'
})

const reset = () => {
    result.value = null
    barcode.value = ''
}

const verifyBarcode = async () => {
    errors.barcode = false
    errors.message = ''

    if (!barcode.value) {
        errors.barcode = true
        return
    }

    processing.value = true
    try {
        const data = await publicFetch('/api/public/purchase/verify', {
            method: 'POST',
            body: {
                code: barcode.value,
                ct_token: ct_token.value,
            }
        });
        if (data) {
            result.value = data;
        } else {
            errors.message = 'Something went wrong!'
        }
    } catch (e) {
        errors.message = e?.data?.message || 'Something went wrong'
    } finally {
        processing.value = false
    }
}
</script>