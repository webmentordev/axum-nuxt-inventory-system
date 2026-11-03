<template>
    <div
        :class="[$attrs.class, 'fixed bottom-3 left-[42%] bg-black border border-input py-2 px-5 rounded-2xl z-50 max-w-85 w-fit']">
        <div class="flex items-center">
            <Icon name="svg-spinners:180-ring-with-bg" class="text-white" size="30px" />
            <p class="text-white ml-3 text-sm">{{ message }}</p>
        </div>
    </div>
</template>
<script setup>
defineProps({
    message: {
        type: String,
        default: "Loading..."
    }
})
</script>