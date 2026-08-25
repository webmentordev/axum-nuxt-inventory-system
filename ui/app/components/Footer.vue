<template>
    <footer class="bg-navy text-white">
        <div class="mx-auto grid max-w-7xl gap-12 px-4 py-14 md:grid-cols-2 lg:grid-cols-4">

            <div>
                <NuxtLink to="/" class="mb-5 text-right leading-none inline-block">
                    <img src="/kaleem-solar-logo-t-4.png" alt="kaleemSolar Multan Logo" width="140px">
                </NuxtLink>

                <p class="text-sm leading-7 text-slate-300 mb-3">
                    Providing quality solar products and complete solar solutions for homes, businesses and industries
                    across Pakistan.
                </p>

                <ul class="text-sm">
                    <li class="mb-1"><strong title="Kaleem solar business days">Open:</strong> Mon - Sun, Friday Closed
                    </li>
                    <li><strong title="Kaleem solar working hours">Hours:</strong> 10AM - 6PM</li>
                </ul>

                <div class="mt-6 flex gap-3">
                    <NuxtLink to="/" class="text-white" target="_blank" title="kaleem solar facebook page">
                        <Icon name="formkit:facebook" size="25px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank" title="kaleem solar instagram profile">
                        <Icon name="mynaui:instagram" size="27px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank" title="kaleem solar youtube channel">
                        <Icon name="mynaui:youtube" size="30px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank" title="kaleem solar tiktok">
                        <Icon name="formkit:tiktok" size="27px" />
                    </NuxtLink>
                </div>
            </div>

            <div>
                <h3 class="mb-5 text-lg font-bold">QUICK LINKS</h3>
                <ul class="space-y-3 text-sm text-slate-300">
                    <li>
                        <NuxtLink to="/" class="hover:text-orange">Home</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/about-us" class="hover:text-orange">About Us</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/track-your-order" class="hover:text-orange">Track your order</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/contact-us" class="hover:text-orange">Contact Us</NuxtLink>
                    </li>
                </ul>
            </div>

            <div>
                <h3 class="mb-5 text-lg font-bold">CATEGORIES</h3>
                <ul class="space-y-3 text-sm text-slate-300">
                    <li v-for="(category, index) in categories" :key="index">
                        <NuxtLink :to='`/categories/${category.slug}`' class="hover:text-orange">{{ category.name }}
                        </NuxtLink>
                    </li>
                </ul>
            </div>

            <div>
                <h3 class="mb-5 text-lg font-bold">HELP & SUPPORT</h3>
                <ul class="space-y-3 text-sm text-slate-300" title="Kaleem solar policies">
                    <li>
                        <NuxtLink to="/shipping-policy" class="hover:text-orange">Shipping Policy</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/warranty-policy" class="hover:text-orange">Warranty Policy</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/return-policy" class="hover:text-orange">Return/Refund Policy</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/terms-of-service" class="hover:text-orange">Terms of service</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/privacy-policy" class="hover:text-orange">Privacy Policy</NuxtLink>
                    </li>
                </ul>
            </div>
        </div>

        <div class="border-t border-slate-700">
            <div
                class="mx-auto flex max-w-7xl flex-col justify-between gap-3 px-4 py-5 text-xs text-slate-400 md:flex-row">
                <p>© 2026 کلیم سولر. All Rights Reserved.</p>
                <div class="flex gap-6">
                    <NuxtLink to="/terms-of-service" class="hover:text-white">Terms of service</NuxtLink>
                    <NuxtLink to="/privacy-policy" class="hover:text-white">Privacy Policy</NuxtLink>
                </div>
            </div>
        </div>
    </footer>
</template>

<script setup lang="js">
const { publicFetch } = usePublicFetch();

const categories = ref([]);

try {
    const data = await publicFetch('/api/public/categories?sub_categories=false&is_featured=true');
    if (data) {
        categories.value = data;
    }
} catch (e) {
    throw createError({
        status: e.statusCode || 500,
        statusText: e.statusMessage || 'Something went wrong!',
        fatal: true
    });
}
</script>