<template>
    <footer class="bg-navy text-white">
        <div
            class="mx-auto grid max-w-7xl gap-10 lg:gap-12 px-4 py-10 lg:py-14 grid-cols-2 md:grid-cols-2 lg:grid-cols-4">

            <div class="col-span-2 lg:col-span-1">
                <NuxtLink to="/" class="mb-5 text-right leading-none inline-block" aria-label="Example mart home page">
                    <img src="/logo-1.webp" alt="Example mart Multan Logo" width="130" height="40">
                </NuxtLink>

                <p class="text-sm leading-7 text-slate-300 mb-3">
                    Your one-stop shop for quality products and everyday essentials, offering a wide range of items to
                    meet all your needs.
                </p>

                <ul class="text-sm">
                    <li class="mb-1"><strong title="Example mart business days">Open:</strong> Mon - Sun, Friday Closed
                    </li>
                    <li><strong title="Example mart working hours">Hours:</strong> 10AM - 6PM</li>
                </ul>

                <div class="mt-6 flex gap-3">
                    <NuxtLink to="/" class="text-white" target="_blank" title="Example mart facebook page">
                        <Icon name="formkit:facebook" size="25px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank" title="Example mart instagram profile">
                        <Icon name="mynaui:instagram" size="27px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank" title="Example mart youtube channel">
                        <Icon name="mynaui:youtube" size="30px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank" title="Example mart tiktok">
                        <Icon name="formkit:tiktok" size="27px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank" title="Example mart twitter">
                        <Icon name="formkit:twitter" size="27px" />
                    </NuxtLink>
                </div>
            </div>

            <div>
                <h3 class="mb-4 lg:mb-5 text-base lg:text-lg font-bold">QUICK LINKS</h3>
                <ul class="space-y-3 text-sm text-slate-300">
                    <li>
                        <NuxtLink to="/" class="hover:text-main">Home</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/products" class="hover:text-main">Products</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/about-us" class="hover:text-main">About Us</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/order/track" class="hover:text-main">Track your order</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/contact-us" class="hover:text-main">Contact Us</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="/verify-purchase" class="hover:text-main">Verify purchase</NuxtLink>
                    </li>
                </ul>
            </div>

            <div v-if="categories.length">
                <h3 class="mb-4 lg:mb-5 text-base lg:text-lg font-bold">CATEGORIES</h3>
                <ul class="space-y-3 text-sm text-slate-300">
                    <li v-for="(category, index) in categories" :key="index">
                        <NuxtLink :to='`/categories/${category.slug}`' class="hover:text-main">{{ category.name }}
                        </NuxtLink>
                    </li>
                </ul>
            </div>

            <div v-if="policies.length">
                <h3 class="mb-4 lg:mb-5 text-base lg:text-lg font-bold">HELP & SUPPORT</h3>
                <ul class="space-y-3 text-sm text-slate-300" title="Example mart policies">
                    <li v-for="policy in policies" :key="policy.slug">
                        <NuxtLink :to="`/policies/${policy.slug}`" class="hover:text-main">
                            {{ policy.name }}
                        </NuxtLink>
                    </li>
                </ul>
            </div>
        </div>

        <div class="border-t border-slate-700">
            <div
                class="mx-auto flex max-w-7xl flex-col justify-between gap-3 px-4 py-5 text-xs text-slate-400 md:flex-row text-center md:text-left">
                <p>© 2026 Mart | All Rights Reserved.</p>
                <p class="flex gap-1 items-center justify-center md:justify-start">Built in
                    <Icon name="selfhst:nuxt-light" size="18" aria-label="Nuxt icon" /> &
                    <Icon name="material-icon-theme:rust" size="18" aria-label="Rust icon" />
                </p>
            </div>
        </div>
    </footer>
</template>
<script setup lang="js">
const { publicFetch } = usePublicFetch();

const { data: categories } = await useAsyncData('categories', () => {
    const params = new URLSearchParams({
        sub_categories: 'false',
        is_featured: 'true',
        with_uploads: 'false',
    });
    return publicFetch(`/api/public/categories?${params}`);
}, { default: () => [] });

const { data: policies } = await useAsyncData('policies', () =>
    publicFetch('/api/public/policies'),
    { default: () => [] }
);
</script>