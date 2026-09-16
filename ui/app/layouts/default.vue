<template>
    <NuxtLoadingIndicator />
    <section class="h-screen flex bg-dark relative overflow-hidden">
        <button
            class="lg:hidden fixed top-3 left-3 z-50 bg-dark-100 border border-dark-300 rounded-md p-2 text-zinc-300"
            @click="isOpen = true">
            <Icon name="mdi:menu" size="24" />
        </button>
        <div v-if="isOpen" class="fixed inset-0 bg-black/60 z-40 lg:hidden" @click="isOpen = false"></div>
        <div class="fixed lg:static top-0 left-0 h-full w-55 shrink-0 border-r border-dark-300 bg-dark-100 p-3 z-50 transition-transform duration-300 lg:translate-x-0"
            :class="isOpen ? 'translate-x-0' : '-translate-x-full'">
            <AdminNavbar />
        </div>
        <div class="w-full h-full overflow-y-auto themed-scroll">
            <slot />
        </div>
    </section>
</template>

<script setup lang="js">
const isOpen = useState('admin-nav-open', () => false);
</script>

<style scoped>
.themed-scroll {
    scrollbar-width: thin;
    scrollbar-color: var(--color-dark-400) transparent;
}

.themed-scroll::-webkit-scrollbar {
    width: 8px;
}

.themed-scroll::-webkit-scrollbar-track {
    background: transparent;
}

.themed-scroll::-webkit-scrollbar-thumb {
    background-color: var(--color-dark-400);
    border-radius: 8px;
}

.themed-scroll::-webkit-scrollbar-thumb:hover {
    background-color: var(--color-lime-dim);
}
</style>