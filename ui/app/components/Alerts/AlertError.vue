<template>
    <p class="text-red-600 mt-2 text-sm ml-2">{{ error }}</p>
</template>
<script setup>
defineProps({
    error: {
        String,
        default: "Input field is required."
    }
})
</script>