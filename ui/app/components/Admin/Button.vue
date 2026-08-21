<template>
    <button
        :class="[$attrs.class, 'px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors flex items-center justify-center']">
        <Icon v-if="icon" :name="icon" size="16" class="mr-1" />
        <span>
            <slot />
        </span>
    </button>
</template>
<script setup lang="js">
defineProps({
    icon: {
        typeof: String,
        default: ""
    }
})
</script>