<template>
    <section class="w-full bg-navy">
        <div class="w-full flex items-center justify-between max-w-7xl m-auto p-3 text-sm">
            <p class="text-white">
                Free & same-day delivery is available exclusively in <strong class="text-orange">Multan</strong>.
            </p>
            <li class="flex items-center gap-2 text-white">
                <Icon name="ic:round-mail" size="20px" class="text-orange" /><strong>{{ config.email }}</strong>
            </li>
        </div>
    </section>
</template>

<script setup lang="js">
const config = useRuntimeConfig().public;
</script>