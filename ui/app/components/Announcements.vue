<template>
    <section class="w-full bg-navy">
        <div class="w-full flex items-center justify-between p-3 text-sm">
            <div class="flex items-center gap-6">
                <li class="flex items-center gap-2 text-white">
                    <Icon name="material-symbols:call-sharp" size="20px" class="text-orange" /><strong>{{ config.phone
                        }}</strong>
                </li>
                <li class="flex items-center gap-2 text-white">
                    <Icon name="ic:round-mail" size="20px" class="text-orange" /><strong>{{ config.email }}</strong>
                </li>
            </div>
            <div class="flex items-center gap-6">
                <div class="text-white flex items-center gap-2">
                    <Icon name="iconoir:delivery-truck" size="22px" />
                    <p>Free delivery exclusively in <strong class="text-orange">Multan</strong>.</p>
                </div>
                <div class="text-white flex items-center gap-2">
                    <Icon name="ri:shield-check-line" size="18px" />
                    <p>100% Original products with warranty.</p>
                </div>
            </div>
        </div>
    </section>
</template>

<script setup lang="js">
const config = useRuntimeConfig().public;
</script>