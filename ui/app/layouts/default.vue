<template>
    <section class="h-screen flex bg-dark">
        <div class="w-55 shrink-0 h-full border-r border-dark-300 bg-dark-100 p-3">
            <AdminNavbar />
        </div>
        <div class="w-full h-full overflow-y-auto themed-scroll">
            <slot />
        </div>
    </section>
</template>

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