<template>
    <section>
        <header class="relative min-h-55 sm:min-h-70 lg:min-h-180 mb-8 lg:mb-12">
            <img src="/mart-banner.webp" alt="" fetchpriority="high"
                class="absolute inset-0 w-full h-full object-cover -z-10">
        </header>
        <div class="max-w-7xl m-auto py-6 px-4" v-if="categories.length > 0">
            <h3 class="uppercase text-center font-semibold text-xl sm:text-2xl text-navy">Shop by category</h3>
            <div class="h-0.5 max-w-25 bg-orange m-auto mt-2"></div>
            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-3 sm:gap-4 my-6">
                <CategoryItem v-for="(category, index) in categories" :category="category" :key="index" />
            </div>
        </div>

        <section class="max-w-7xl m-auto py-6 px-4" v-if="products.length > 0">
            <h3 class="uppercase text-center font-semibold text-xl sm:text-2xl text-navy">Latest arrivals</h3>
            <div class="h-0.5 max-w-25 bg-orange m-auto mt-2"></div>
            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-3 sm:gap-4 my-6">
                <ProductsCard v-for="product in products" :product="product" :key="product.id" />
            </div>
            <NuxtLink to="/products"
                class="uppercase flex items-center w-fit m-auto py-2 px-3 rounded-md border border-navy">
                <strong class="text-navy text-sm">View all products</strong>
                <Icon name="ri:apps-ai-line" size="22px" class="text-navy ml-2" />
            </NuxtLink>
        </section>

        <section
            class="mx-auto grid max-w-7xl items-center gap-8 lg:gap-12 px-4 pb-16 lg:pb-20 grid-cols-1 lg:grid-cols-2 py-8 lg:py-12">
            <div class="relative">
                <img src="/mart-image.webp" class="h-64 sm:h-80 lg:h-105 w-full rounded-2xl object-cover shadow-xl"
                    alt="Mart store shelves image">

                <div class="absolute left-3 top-3 sm:left-5 sm:top-5 rounded-xl bg-white p-3 sm:p-5 shadow-xl">
                    <strong class="text-xl sm:text-3xl text-main-dark">500+</strong>
                    <p class="text-xs sm:text-sm font-semibold text-navy">Happy Customers</p>
                </div>

                <div class="absolute bottom-3 right-3 sm:bottom-5 sm:right-5 rounded-xl bg-white p-3 sm:p-5 shadow-xl">
                    <strong class="text-xl sm:text-3xl text-main-dark">1000+</strong>
                    <p class="text-xs sm:text-sm font-semibold text-navy">Products Available</p>
                </div>
            </div>
            <div>
                <p class="font-bold uppercase tracking-wide text-main-dark text-md sm:text-base">Why Choose Us?</p>
                <h2 class="mt-2 text-2xl sm:text-3xl lg:text-4xl font-black text-navy">
                    Your Trusted Partner
                    <br>
                    for Everyday Shopping
                </h2>
                <div class="mt-6 lg:mt-8 space-y-5 lg:space-y-6">
                    <div class="flex gap-4 sm:gap-5 border-b pb-5">
                        <span class="text-2xl sm:text-3xl text-main-dark">🛒</span>
                        <div>
                            <h3 class="font-bold text-navy text-sm sm:text-base">Wide Product Range</h3>
                            <p class="mt-1 text-xs sm:text-sm text-slate-500">Find everything you need, from groceries
                                to
                                household essentials, in one place.</p>
                        </div>
                    </div>
                    <div class="flex gap-4 sm:gap-5 border-b pb-5">
                        <span class="text-2xl sm:text-3xl text-main">🏅</span>
                        <div>
                            <h3 class="font-bold text-navy text-sm sm:text-base">Quality You Can Trust</h3>
                            <p class="mt-1 text-xs sm:text-sm text-slate-500">We offer fresh, top quality products from
                                trusted brands.</p>
                        </div>
                    </div>
                    <div class="flex gap-4 sm:gap-5 border-b pb-5">
                        <span class="text-2xl sm:text-3xl text-main">🏷</span>
                        <div>
                            <h3 class="font-bold text-navy text-sm sm:text-base">Affordable Prices</h3>
                            <p class="mt-1 text-xs sm:text-sm text-slate-500">Great value on everyday items with regular
                                deals and discounts.</p>
                        </div>
                    </div>
                    <div class="flex gap-4 sm:gap-5">
                        <span class="text-2xl sm:text-3xl text-main">👥</span>
                        <div>
                            <h3 class="font-bold text-navy text-sm sm:text-base">Friendly Customer Service</h3>
                            <p class="mt-1 text-xs sm:text-sm text-slate-500">Our dedicated team is always here to help
                                you.</p>
                        </div>
                    </div>
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

        <section class="border-b bg-white">
            <div class="mx-auto grid max-w-7xl grid-cols-1 sm:grid-cols-2 md:grid-cols-4">
                <div class="flex items-center gap-4 p-6">
                    <span class="text-3xl text-main">🛡</span>
                    <div>
                        <h3 class="font-bold text-navy">100% Secure Payments</h3>
                        <p class="text-sm text-slate-500">Your payments are safe and secure.</p>
                    </div>
                </div>
                <div class="flex items-center gap-4 p-6">
                    <span class="text-3xl text-navy">↩</span>
                    <div>
                        <h3 class="font-bold text-navy">Easy Returns</h3>
                        <p class="text-sm text-slate-500">7 days easy return policy.</p>
                    </div>
                </div>
                <div class="flex items-center gap-4 p-6">
                    <span class="text-3xl text-main">🚚</span>
                    <div>
                        <h3 class="font-bold text-navy">Fast Delivery</h3>
                        <p class="text-sm text-slate-500">Delivery all over Pakistan.</p>
                    </div>
                </div>
                <div class="flex items-center gap-4 p-6">
                    <span class="text-3xl text-main">🎧</span>
                    <div>
                        <h3 class="font-bold text-navy">24/7 Support</h3>
                        <p class="text-sm text-slate-500">We are available 24/7.</p>
                    </div>
                </div>
            </div>
        </section>
    </section>
</template>

<script setup lang="js">
definePageMeta({
    layout: 'home'
});

const { publicFetch } = usePublicFetch();

const { data: products } = await useAsyncData('home-products-limited', () =>
    publicFetch('/api/public/products/limited'),
    { default: () => [] }
);

const { data: categories } = await useAsyncData('home-categories', () => {
    const params = new URLSearchParams({
        sub_categories: 'false',
        is_featured: 'true',
        with_uploads: 'true',
        limit: '5'
    });
    return publicFetch(`/api/public/categories?${params}`);
}, { default: () => [] });
</script>