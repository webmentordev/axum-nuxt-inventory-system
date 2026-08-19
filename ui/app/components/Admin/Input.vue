<template>
    <input :value="props.modelValue" @input="emit('update:modelValue', $event.target.value)" autocomplete="off"
        :class="[$attrs.class, 'p-3 border border-dark-300 bg-dark-200 outline-none placeholder:text-zinc-500 focus:outline-none focus:border-lime-main/50 focus:ring-2 focus:ring-lime-main/20 w-full text-zinc-200 text-sm rounded-xl transition-colors']"
        v-bind="{ ...$attrs, class: undefined }">
</template>

<script setup>
const props = defineProps(['modelValue'])
const emit = defineEmits(['update:modelValue'])
</script>