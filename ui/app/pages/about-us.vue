<template>
    <section class="w-full">
        <header class="bg-navy text-white py-14 sm:py-20 px-4 text-center">
            <p class="font-bold uppercase tracking-wide text-orange text-sm sm:text-base">About Us</p>
            <h1 class="mt-2 text-2xl sm:text-4xl font-black">Your Trusted Partner in Solar Energy</h1>
            <p class="mt-4 max-w-2xl mx-auto text-sm sm:text-base text-slate-300">
                Providing quality solar products and complete solar solutions for homes, businesses and industries
                across Pakistan.
            </p>
        </header>

        <section
            class="max-w-5xl mx-auto px-4 py-10 sm:py-14 grid grid-cols-2 sm:grid-cols-4 gap-4 sm:gap-6 text-center">
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-orange">500+</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Happy Customers</p>
            </div>
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-orange">1000+</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Projects Completed</p>
            </div>
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-orange">100%</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Original Products</p>
            </div>
            <div class="rounded-xl border border-gray-200 p-4 sm:p-6">
                <strong class="text-2xl sm:text-3xl text-orange">24/7</strong>
                <p class="mt-1 text-xs sm:text-sm font-semibold text-navy">Customer Support</p>
            </div>
        </section>

        <section class="bg-slate-50 py-10 sm:py-14 px-4">
            <div class="max-w-5xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 items-center">
                <img src="https://images.unsplash.com/photo-1509391366360-2e959784a276?auto=format&fit=crop&w=1200&q=80"
                    class="h-64 sm:h-80 lg:h-96 w-full rounded-2xl object-cover shadow-xl" alt="Solar installation">
                <div>
                    <p class="font-bold uppercase tracking-wide text-orange text-sm sm:text-base">Who We Are</p>
                    <h2 class="mt-2 text-2xl sm:text-3xl font-black text-navy">Powering Multan with Clean Energy</h2>
                    <p class="mt-4 text-sm sm:text-base text-slate-500 leading-7">
                        Kaleem Solar is a Multan based solar products retailer, dedicated to helping homes and
                        businesses make the switch to renewable energy. From solar panels and inverters to complete
                        installation solutions, we offer top quality products from trusted global brands at
                        competitive prices, backed by expert guidance and dedicated after sales support.
                    </p>
                </div>
            </div>
        </section>

        <section class="max-w-7xl mx-auto px-4 py-10 sm:py-14">
            <h3 class="uppercase text-center font-semibold text-xl sm:text-2xl text-navy">Why Choose Us</h3>
            <div class="h-0.5 max-w-25 bg-orange m-auto mt-2"></div>
            <div class="mt-8 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                <div class="text-center">
                    <span class="text-3xl text-orange">🛡</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">Expert Guidance</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">Get professional advice to choose the right
                        solar solution.</p>
                </div>
                <div class="text-center">
                    <span class="text-3xl text-orange">🏅</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">Quality You Can Trust</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">We offer top quality products from trusted
                        global brands.</p>
                </div>
                <div class="text-center">
                    <span class="text-3xl text-orange">🏷</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">Affordable Prices</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">Premium quality solar products at competitive
                        prices.</p>
                </div>
                <div class="text-center">
                    <span class="text-3xl text-orange">👥</span>
                    <h3 class="mt-3 font-bold text-navy text-sm sm:text-base">After Sales Support</h3>
                    <p class="mt-1 text-xs sm:text-sm text-slate-500">Our dedicated support team is always here for
                        you.</p>
                </div>
            </div>
        </section>

        <section class="relative overflow-hidden bg-navy py-12 sm:py-14 text-white">
            <div
                class="relative mx-auto flex max-w-7xl flex-col items-center justify-between gap-6 lg:gap-8 px-4 md:flex-row">
                <div class="flex flex-col sm:flex-row items-center gap-4 sm:gap-6 text-center sm:text-left">
                    <img src="/kaleem-solar-logo-t-4.png" width="120px">
                    <div>
                        <h2 class="text-2xl sm:text-3xl font-black">Ready to Switch to Solar?</h2>
                        <p class="mt-2 text-sm sm:text-base text-slate-300">Let us help you save money and power your
                            future with clean, renewable energy.</p>
                    </div>
                </div>
                <div class="flex flex-wrap gap-4">
                    <NuxtLink to="/contact-us"
                        class="rounded-lg border border-white px-6 py-3 sm:px-7 sm:py-4 font-bold text-sm sm:text-base">
                        CONTACT US ☎
                    </NuxtLink>
                </div>
            </div>
        </section>
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'home'
});

</script>