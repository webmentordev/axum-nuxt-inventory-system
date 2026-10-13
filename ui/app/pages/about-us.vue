<template>
    <section class="w-full">
        <header class="bg-navy text-white py-14 sm:py-20 px-4 text-center">
            <p class="font-bold uppercase tracking-wide text-main text-sm sm:text-base">About Us</p>
            <h1 class="mt-2 text-2xl sm:text-4xl font-black">Your Trusted Partner for Everyday Shopping</h1>
            <p class="mt-4 max-w-2xl mx-auto text-sm sm:text-base text-slate-300">
                Providing quality products and a convenient shopping experience for families and businesses
                across Pakistan.
            </p>
        </header>

        <section
            class="max-w-5xl mx-auto px-4 py-10 sm:py-14 grid grid-cols-2 sm:grid-cols-4 gap-4 sm:gap-6 text-center">
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-main">500+</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Happy Customers</p>
            </div>
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-main">1000+</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Products Available</p>
            </div>
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-main">100%</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Original Products</p>
            </div>
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-main">24/7</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Customer Support</p>
            </div>
        </section>

        <section class="bg-slate-50 py-10 sm:py-14 px-4">
            <div class="max-w-5xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 items-center">
                <img src="/mart-image.webp" class="h-64 sm:h-80 lg:h-96 w-full rounded-2xl object-cover shadow-xl"
                    alt="Mart store">
                <div>
                    <p class="font-bold uppercase tracking-wide text-main text-sm sm:text-base">Who We Are</p>
                    <h2 class="mt-2 text-2xl sm:text-3xl font-black text-navy">Serving Our Community with Quality</h2>
                    <p class="mt-4 text-sm sm:text-base text-slate-500 leading-7">
                        Example Mart is a trusted local retail store, dedicated to making everyday shopping simple
                        and affordable. From groceries and household essentials to daily-use items, we offer top
                        quality products from trusted brands at competitive prices, backed by friendly service and
                        dedicated customer support.
                    </p>
                </div>
            </div>
        </section>

        <section class="max-w-7xl mx-auto px-4 py-10 sm:py-14">
            <h3 class="uppercase text-center font-semibold text-xl sm:text-2xl text-navy">Why Choose Us</h3>
            <div class="h-0.5 max-w-25 bg-orange m-auto mt-2"></div>
            <div class="mt-8 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                <div class="text-center">
                    <span class="text-3xl text-main">🛒</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">Wide Product Range</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">Find everything you need, from groceries to
                        household essentials, in one place.</p>
                </div>
                <div class="text-center">
                    <span class="text-3xl text-main">🏅</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">Quality You Can Trust</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">We offer fresh, top quality products from
                        trusted brands.</p>
                </div>
                <div class="text-center">
                    <span class="text-3xl text-main">🏷</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">Affordable Prices</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">Great value on everyday items with regular
                        deals and discounts.</p>
                </div>
                <div class="text-center">
                    <span class="text-3xl text-main">👥</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">Friendly Customer Service</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">Our dedicated team is always here to help
                        you.</p>
                </div>
            </div>
        </section>

        <section class="relative overflow-hidden bg-navy py-10 lg:py-14 text-white">
            <div class="absolute inset-0 opacity-10">
                <img src="/mart-image.webp" class="h-full w-full object-cover" alt="Mart store image">
            </div>
            <div
                class="relative mx-auto flex max-w-7xl flex-col items-center justify-between gap-6 lg:gap-8 px-4 md:flex-row">
                <div class="flex flex-col sm:flex-row items-center gap-4 sm:gap-6 text-center sm:text-left">
                    <div class="leading-none">
                        <img src="/logo-1.webp" width="130" height="40" alt="Example Mart">
                    </div>
                    <div>
                        <h2 class="text-2xl sm:text-3xl font-black">Ready to Start Shopping?</h2>
                        <p class="mt-2 text-sm sm:text-base text-slate-300">Visit us today and discover quality
                            products,
                            great prices, and a better shopping experience.</p>
                    </div>
                </div>
                <div class="flex flex-wrap gap-4">
                    <a href="/contact-us"
                        class="rounded-lg border border-white px-6 py-3 sm:px-7 sm:py-4 font-bold text-sm sm:text-base">CONTACT
                        US ☎</a>
                </div>
            </div>
        </section>
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'home'
});

const config = useRuntimeConfig();
const canonicalUrl = `${config.public.siteUrl}/about-us`;

useSeoMeta({
    title: 'About Us | Example Mart',
    description: 'Example Mart is a trusted local retail store offering quality groceries, household essentials and everyday products at affordable prices with friendly customer service.',
    keywords: 'About Example Mart, local mart, grocery store, everyday shopping, household essentials',
    ogTitle: 'About Us | Example Mart',
    ogDescription: 'Example Mart is a trusted local retail store offering quality groceries, household essentials and everyday products at affordable prices with friendly customer service.',
    ogImage: `${config.public.siteUrl}/mart-image.webp`,
    ogUrl: canonicalUrl,
    ogType: 'website',
    twitterCard: 'summary_large_image',
    twitterTitle: 'About Us | Example Mart',
    twitterDescription: 'Example Mart is a trusted local retail store offering quality groceries, household essentials and everyday products at affordable prices with friendly customer service.'
});

useHead({
    link: [
        {
            rel: 'canonical',
            href: canonicalUrl
        }
    ]
});
</script>