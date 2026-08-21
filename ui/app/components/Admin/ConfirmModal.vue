<template>
    <Teleport to="body">
        <Transition name="confirm-modal">
            <div v-if="modelValue" class="fixed inset-0 z-50 flex items-center justify-center px-4">
                <div class="absolute inset-0 bg-black/60" @click="cancel"></div>

                <div class="relative w-full max-w-sm rounded-xl border border-dark-300 bg-dark-100 p-5 shadow-lg">
                    <h2 class="text-base font-bold text-white">{{ title }}</h2>
                    <p class="text-sm text-zinc-400 mt-2">{{ message }}</p>

                    <div class="flex items-center justify-end gap-2 mt-5">
                        <button type="button" @click="cancel"
                            class="px-4 py-2 rounded-md text-sm font-semibold text-zinc-300 hover:bg-dark-300 hover:text-white transition-colors">
                            {{ cancelText }}
                        </button>
                        <button type="button" @click="confirm"
                            class="px-4 py-2 rounded-md text-sm font-semibold bg-red-500 text-white hover:bg-red-400 transition-colors">
                            {{ confirmText }}
                        </button>
                    </div>
                </div>
            </div>
        </Transition>
    </Teleport>
</template>

<script setup>
const props = defineProps({
    modelValue: {
        type: Boolean,
        default: false
    },
    title: {
        type: String,
        default: 'Are you sure?'
    },
    message: {
        type: String,
        default: 'This action cannot be undone.'
    },
    confirmText: {
        type: String,
        default: 'Delete'
    },
    cancelText: {
        type: String,
        default: 'Cancel'
    }
})

const emit = defineEmits(['update:modelValue', 'confirm', 'cancel'])

function confirm() {
    emit('update:modelValue', false)
    emit('confirm')
}

function cancel() {
    emit('update:modelValue', false)
    emit('cancel')
}
</script>

<style scoped>
.confirm-modal-enter-active,
.confirm-modal-leave-active {
    transition: opacity 0.15s ease;
}

.confirm-modal-enter-from,
.confirm-modal-leave-to {
    opacity: 0;
}

.confirm-modal-enter-active .relative,
.confirm-modal-leave-active .relative {
    transition: transform 0.15s ease, opacity 0.15s ease;
}

.confirm-modal-enter-from .relative,
.confirm-modal-leave-to .relative {
    transform: scale(0.96);
    opacity: 0;
}
</style>