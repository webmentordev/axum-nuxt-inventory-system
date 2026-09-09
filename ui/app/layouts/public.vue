<template>
    <Announcements />
    <Navbar />
    <div class="max-w-7xl m-auto w-full py-3 px-4 grid grid-cols-1 lg:grid-cols-8 gap-3 min-h-[80vh]">
        <AppCategories class="lg:col-span-2" />
        <div class="lg:col-span-6 w-full h-full">
            <slot />
        </div>
    </div>
    <Footer />
</template>