<template>
    <button
        :class="[$attrs.class, 'bg-navy text-white w-full py-3 rounded-xl flex items-center justify-center hover:bg-navy/90 group']">
        <span>{{ text }}</span>
    </button>
</template>

<script setup lang="js">
defineProps({
    text: {
        type: String
    },
});
</script>