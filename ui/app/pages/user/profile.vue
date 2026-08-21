<template>
    <section class="w-full ">
        <div class="max-w-2xl m-auto p-3">
            <h1>Welcome to profile page!</h1>
        </div>
    </section>
</template>

<script setup>
definePageMeta({
    middleware: 'auth',
    layout: 'client'
});
</script>