<template>
    <section class="w-full h-[80vh]">
        <div class="flex items-center max-w-2xl m-auto w-full p-3">
            <h2>{{ error?.status }}</h2>
            <span class="ml-1 pl-1 border-l border-gray-200">{{ error?.statusText }}</span>
        </div>
        <Button text="Clear errors" />
    </section>
</template>

<script setup lang="ts">
const props = defineProps({
    error: Object as () => NuxtError,
})

const handleError = () => clearError({ redirect: '/' })
</script>