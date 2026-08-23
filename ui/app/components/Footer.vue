<template>
    <footer class="bg-navy text-white">
        <div class="mx-auto grid max-w-7xl gap-12 px-4 py-14 md:grid-cols-2 lg:grid-cols-4">

            <div>
                <NuxtLink to="/" class="mb-5 text-right leading-none inline-block">
                    <img src="/kaleem-solar-logo-t-4.png" alt="kaleemSolar Multan Logo" width="140px">
                </NuxtLink>

                <p class="text-sm leading-7 text-slate-300">
                    Providing quality solar products and complete solar solutions for homes, businesses and industries
                    across Pakistan.
                </p>

                <div class="mt-6 flex gap-3">
                    <NuxtLink to="/" class="text-white" target="_blank">
                        <Icon name="formkit:facebook" size="25px" />
                    </NuxtLink>
                    <NuxtLink to="/" class="text-white" target="_blank">
                        <Icon name="mynaui:instagram" size="27px" />
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
                    <li>
                        <NuxtLink to="#" class="hover:text-orange">Solar Panels</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="#" class="hover:text-orange">Inverters</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="#" class="hover:text-orange">Batteries</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="#" class="hover:text-orange">Solar Structures</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="#" class="hover:text-orange">Cables</NuxtLink>
                    </li>
                    <li>
                        <NuxtLink to="#" class="hover:text-orange">Connectors</NuxtLink>
                    </li>
                </ul>
            </div>

            <div>
                <h3 class="mb-5 text-lg font-bold">HELP & SUPPORT</h3>
                <ul class="space-y-3 text-sm text-slate-300">
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