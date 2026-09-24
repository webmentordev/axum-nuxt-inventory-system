<template>
    <Announcements />
    <Navbar />
    <main id="main-content" role="main">
        <div class="max-w-7xl m-auto w-full py-3 px-4 min-h-[80vh] h-full">
            <slot />
        </div>
    </main>
    <Footer />
</template>