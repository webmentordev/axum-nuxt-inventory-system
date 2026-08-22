<template>
    <section class="w-full h-[80vh]">
        <div class="max-w-2xl m-auto p-3">
            <h1>Welcome to {{ slug }} product page!</h1>
        </div>
    </section>
</template>

<script setup>
definePageMeta({
    layout: 'public'
});

const route = useRoute();
const slug = route.params.slug;

</script>