<template>
    <section>
        <header class="min-h-180 bg-cover bg-center mb-12" style="background-image: url('/kaleem-solar-banner.png');">
        </header>
        <div class="max-w-7xl m-auto py-6" v-if="categories.length > 0">
            <h3 class="uppercase text-center font-semibold text-2xl text-navy">Shop by category</h3>
            <div class="h-0.5 max-w-25 bg-orange m-auto mt-2"></div>
            <div class="grid grid-cols-5 gap-4 my-6">
                <CategoryItem v-for="(category, index) in categories" :category="category" :key="index" />
            </div>
        </div>

        <section class="max-w-7xl m-auto py-6" v-if="products.length > 0">
            <h3 class="uppercase text-center font-semibold text-2xl text-navy">Latest arrivals</h3>
            <div class="h-0.5 max-w-25 bg-orange m-auto mt-2"></div>
            <div class="grid grid-cols-5 gap-4 my-6">
                <ProductsCard v-for="product in products" :product="product" :key="product.id" />
            </div>
            <NuxtLink to="/products"
                class="uppercase flex items-center w-fit m-auto py-2 px-3 rounded-md border border-navy">
                <strong class="text-navy text-sm">View all products</strong>
                <Icon name="ri:apps-ai-line" size="22px" class="text-navy ml-2" />
            </NuxtLink>
        </section>

        <section class="mx-auto grid max-w-7xl items-center gap-12 px-4 pb-20 lg:grid-cols-2 py-12">
            <div class="relative">
                <img src="https://images.unsplash.com/photo-1509391366360-2e959784a276?auto=format&fit=crop&w=1200&q=80"
                    class="h-105 w-full rounded-2xl object-cover shadow-xl" alt="Solar installation">

                <div class="absolute left-5 top-5 rounded-xl bg-white p-5 shadow-xl">
                    <strong class="text-3xl text-orange">500+</strong>
                    <p class="text-sm font-semibold text-navy">Happy Customers</p>
                </div>

                <div class="absolute bottom-5 right-5 rounded-xl bg-white p-5 shadow-xl">
                    <strong class="text-3xl text-orange">1000+</strong>
                    <p class="text-sm font-semibold text-navy">Projects Completed</p>
                </div>
            </div>
            <div>
                <p class="font-bold uppercase tracking-wide text-orange">Why Choose Us?</p>
                <h2 class="mt-2 text-4xl font-black text-navy">
                    Your Trusted Partner
                    <br>
                    in Solar Energy
                </h2>
                <div class="mt-8 space-y-6">
                    <div class="flex gap-5 border-b pb-5">
                        <span class="text-3xl text-orange">🛡</span>
                        <div>
                            <h3 class="font-bold text-navy">Expert Guidance</h3>
                            <p class="mt-1 text-sm text-slate-500">Get professional advice to choose the right solar
                                solution.</p>
                        </div>
                    </div>
                    <div class="flex gap-5 border-b pb-5">
                        <span class="text-3xl text-orange">🏅</span>
                        <div>
                            <h3 class="font-bold text-navy">Quality You Can Trust</h3>
                            <p class="mt-1 text-sm text-slate-500">We offer top quality products from trusted global
                                brands.</p>
                        </div>
                    </div>
                    <div class="flex gap-5 border-b pb-5">
                        <span class="text-3xl text-orange">🏷</span>
                        <div>
                            <h3 class="font-bold text-navy">Affordable Prices</h3>
                            <p class="mt-1 text-sm text-slate-500">Premium quality solar products at competitive prices.
                            </p>
                        </div>
                    </div>
                    <div class="flex gap-5">
                        <span class="text-3xl text-orange">👥</span>
                        <div>
                            <h3 class="font-bold text-navy">After Sales Support</h3>
                            <p class="mt-1 text-sm text-slate-500">Our dedicated support team is always here for you.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="relative overflow-hidden bg-navy py-14 text-white">
            <div class="absolute inset-0 opacity-10">
                <img src="https://images.unsplash.com/photo-1508514177221-188b1cf16e9d?auto=format&fit=crop&w=2000&q=80"
                    class="h-full w-full object-cover" alt="">
            </div>
            <div class="relative mx-auto flex max-w-7xl flex-col items-center justify-between gap-8 px-4 md:flex-row">
                <div class="flex items-center gap-6">
                    <div class="text-right leading-none">
                        <img src="/kaleem-solar-logo-t-4.png" width="120px">
                    </div>
                    <div>
                        <h2 class="text-3xl font-black">Ready to Switch to Solar?</h2>
                        <p class="mt-2 text-slate-300">Let us help you save money and power your future with clean,
                            renewable energy.</p>
                    </div>
                </div>
                <div class="flex flex-wrap gap-4">
                    <a href="/contact-us" class="rounded-lg border border-white px-7 py-4 font-bold">CONTACT US ☎</a>
                </div>
            </div>
        </section>

        <section class="border-b bg-white">
            <div class="mx-auto grid max-w-7xl md:grid-cols-4">
                <div class="flex items-center gap-4 p-6">
                    <span class="text-3xl text-orange">🛡</span>
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
                    <span class="text-3xl text-orange">🚚</span>
                    <div>
                        <h3 class="font-bold text-navy">Fast Delivery</h3>
                        <p class="text-sm text-slate-500">Delivery all over Multan, Pakistan.</p>
                    </div>
                </div>
                <div class="flex items-center gap-4 p-6">
                    <span class="text-3xl text-orange">🎧</span>
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

const products = ref([]);

const { categories, processing, fetchCategories } = useCategories({
    featured: true,
    withSubCategories: false,
    withUploads: true,
    limit: 5
});

onMounted(async () => {
    try {
        const data = await publicFetch('/api/public/products/limited');
        if (data) {
            products.value = data;
        }
    } catch (e) {
        throw createError({
            status: e.statusCode || 500,
            statusText: e.statusMessage || 'Something went wrong!',
            fatal: true
        });
    } finally {
        processing.value = false;
    }

    await fetchCategories();
});
</script>