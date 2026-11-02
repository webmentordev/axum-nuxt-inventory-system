<template>
    <div class="fixed left-2 bottom-2 bg-black border border-white/10 rounded-xl max-w-85 w-full z-50">
        <div class="flex items-center justify-between relative py-3 px-4">
            <div class="flex items-center">
                <Icon name="oui:cross-in-circle-empty" class="text-red-600" size="15px" alt="Error" />
                <div class="flex flex-col ml-3 border-l border-white/10 pl-3">
                    <h3 class="text-red-500">An error occurred!</h3>
                    <p class="text-[12px] text-white">{{ message }}</p>
                </div>
            </div>
            <button @click="closeMessage" class="ml-4 text-white/50 hover:text-white transition absolute top-2 right-2">
                <Icon name="ic:baseline-close" class="text-white" size="18px" />
            </button>
        </div>
    </div>
</template>

<script setup>
defineProps({
    message: {
        type: String,
        default: "Operation failed!"
    }
})

const emit = defineEmits(['close'])

function closeMessage() {
    emit('close')
}
</script>