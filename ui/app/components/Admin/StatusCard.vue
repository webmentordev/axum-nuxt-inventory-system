<template>
    <Transition name="status-card">
        <div v-if="modelValue"
            class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 flex items-center gap-3 px-4 py-3 rounded-xl border text-sm font-semibold shadow-lg max-w-md w-max"
            :class="variants[type].wrapper">
            <Icon v-if="type !== 'loading'" :name="variants[type].icon" size="20" class="shrink-0" />
            <span v-else
                class="shrink-0 w-5 h-5 border-2 border-zinc-500 border-t-lime-main rounded-full animate-spin"></span>
            <span>{{ message || variants[type].default }}</span>
            <button type="button" @click="close"
                class="shrink-0 ml-1 text-current opacity-60 hover:opacity-100 transition-opacity">
                <Icon name="mdi-light:close" size="18" />
            </button>
        </div>
    </Transition>
</template>

<script setup>
const props = defineProps({
    modelValue: {
        type: Boolean,
        default: false
    },
    type: {
        type: String,
        default: 'loading',
        validator: (value) => ['success', 'error', 'loading'].includes(value)
    },
    message: {
        type: String,
        default: ''
    },
    duration: {
        type: Number,
        default: 5000
    }
})

const emit = defineEmits(['update:modelValue'])

const variants = {
    success: {
        wrapper: 'bg-dark-200 border-lime-main/30 text-lime-main',
        icon: 'mdi-light:check-circle',
        default: 'Action completed successfully.'
    },
    error: {
        wrapper: 'bg-dark-200 border-red-500/30 text-red-400',
        icon: 'mdi-light:alert-circle',
        default: 'Something went wrong. Please try again.'
    },
    loading: {
        wrapper: 'bg-dark-200 border-dark-300 text-zinc-300',
        icon: '',
        default: 'Processing, please wait...'
    }
}

let timer = null

const close = () => {
    emit('update:modelValue', false)
}

const startTimer = () => {
    clearTimeout(timer)
    if (props.type !== 'loading' && props.duration > 0) {
        timer = setTimeout(close, props.duration)
    }
}

watch(() => props.modelValue, (visible) => {
    if (visible) startTimer()
    else clearTimeout(timer)
})

onBeforeUnmount(() => clearTimeout(timer))
</script>

<style scoped>
.status-card-enter-active,
.status-card-leave-active {
    transition: opacity 0.2s ease, transform 0.2s ease;
}

.status-card-enter-from,
.status-card-leave-to {
    opacity: 0;
    transform: translate(-50%, 8px);
}
</style>