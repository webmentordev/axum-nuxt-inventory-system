<template>
    <div ref="rootEl" class="relative w-full">
        <button type="button" @click="toggleOpen"
            :class="[$attrs.class, 'p-3 border border-dark-300 bg-dark-200 outline-none focus:outline-none focus:border-lime-main/50 focus:ring-2 focus:ring-lime-main/20 w-full text-sm rounded-xl transition-colors flex items-center justify-between gap-2']"
            v-bind="{ ...$attrs, class: undefined }">
            <span :class="selectedLabel ? 'text-zinc-200' : 'text-zinc-500'">
                {{ selectedLabel || placeholder }}
            </span>
            <Icon name="mdi:chevron-down" size="18" class="text-zinc-500 shrink-0 transition-transform"
                :class="{ 'rotate-180': isOpen }" />
        </button>

        <div v-if="isOpen"
            class="absolute left-0 right-0 top-full mt-1 rounded-xl border border-dark-300 bg-dark-200 shadow-lg z-40 overflow-hidden">
            <div class="p-2 border-b border-dark-300">
                <input ref="searchInput" v-model="search" type="text" autocomplete="off" placeholder="Search..."
                    class="w-full px-2 py-1.5 bg-dark-300 rounded-md text-sm text-zinc-200 placeholder:text-zinc-500 outline-none" />
            </div>

            <ul class="max-h-56 overflow-y-auto">
                <li v-if="!filteredOptions.length" class="px-3 py-2 text-sm text-zinc-500">
                    No results found.
                </li>
                <li v-for="option in filteredOptions" :key="option.value" @click="selectOption(option)"
                    class="px-3 py-2 text-sm cursor-pointer transition-colors" :class="option.value === modelValue
                        ? 'bg-lime-bg text-lime-main'
                        : 'text-zinc-300 hover:bg-dark-300 hover:text-white'">
                    {{ option.label }}
                </li>
            </ul>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, nextTick } from 'vue'
import { onClickOutside } from '@vueuse/core'

const props = defineProps({
    modelValue: {
        type: [String, Number, null],
        default: null
    },
    options: {
        type: Array,
        default: () => []
    },
    placeholder: {
        type: String,
        default: 'Select an option'
    }
})

const emit = defineEmits(['update:modelValue'])

const isOpen = ref(false)
const search = ref('')
const searchInput = ref(null)
const rootEl = ref(null)

onClickOutside(rootEl, () => {
    isOpen.value = false
})

const selectedLabel = computed(() => {
    const match = props.options.find((option) => option.value === props.modelValue)
    return match ? match.label : ''
})

const filteredOptions = computed(() => {
    if (!search.value.trim()) return props.options
    const query = search.value.trim().toLowerCase()
    return props.options.filter((option) => option.label.toLowerCase().includes(query))
})

function toggleOpen() {
    isOpen.value = !isOpen.value
    if (isOpen.value) {
        search.value = ''
        nextTick(() => searchInput.value?.focus())
    }
}

function selectOption(option) {
    emit('update:modelValue', option.value)
    isOpen.value = false
}
</script>