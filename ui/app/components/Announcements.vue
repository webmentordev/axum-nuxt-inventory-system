<template>
    <section class="w-full bg-navy">
        <div
            class="w-full flex flex-col sm:flex-row items-center justify-between gap-2 sm:gap-0 p-2 sm:p-3 text-xs sm:text-sm">
            <div class="flex items-center gap-4 sm:gap-6">
                <li class="flex items-center gap-2 text-white">
                    <Icon name="material-symbols:call-sharp" size="18px" class="text-orange shrink-0" /><strong>{{
                        config.phone
                        }}</strong>
                </li>
                <li class="hidden sm:flex items-center gap-2 text-white">
                    <Icon name="ic:round-mail" size="20px" class="text-orange" /><strong>{{ config.email }}</strong>
                </li>
            </div>
            <div class="hidden md:flex items-center gap-6">
                <div class="text-white flex items-center gap-2">
                    <Icon name="iconoir:delivery-truck" size="22px" />
                    <p>Free delivery exclusively in <strong class="text-orange">Multan</strong>.</p>
                </div>
            </div>
        </div>
    </section>
</template>

<script setup lang="js">
const config = useRuntimeConfig().public;
</script>