<template>
    <section class="h-full w-full">
        <h1>Welcome to dashboard!</h1>
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});
</script>