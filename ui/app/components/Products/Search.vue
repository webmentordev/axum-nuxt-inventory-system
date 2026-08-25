<template>
    <section class="w-full max-w-3xl m-auto flex items-center">
        <input type="search" v-model="search" placeholder="Search by name, brand..."
            class="bg-slate-200 rounded-full w-full py-2 outline-none px-5 text-sm">
        <button class="bg-navy shrink-0 flex items-center text-white py-2 px-4 rounded-full ml-2"><img
                src="https://api.iconify.design/ic:outline-search.svg?color=%23ffffff" width="18px">
            <strong class="text-sm ml-1 -translate-y-0.5">Search</strong>
        </button>
    </section>
</template>

<script setup lang="js">
const search = ref("");
</script>