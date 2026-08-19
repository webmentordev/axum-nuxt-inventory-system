<template>
    <section class="h-full w-full p-6">
        <div class="flex items-center justify-between mb-6">
            <div>
                <h1 class="text-xl font-bold text-white">Products</h1>
                <p class="text-sm text-zinc-500 mt-1">{{ products.length }} total</p>
            </div>
            <NuxtLink to="/admin/products/create"
                class="px-4 py-2 rounded-md text-sm font-semibold bg-lime-main text-dark hover:bg-lime-hover transition-colors">
                Add Product
            </NuxtLink>
        </div>

        <div class="w-full border border-dark-300 rounded-lg overflow-hidden bg-dark-100">
            <table v-if="products.length" class="w-full text-sm">
                <thead class="bg-dark-200">
                    <tr>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Name</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">SKU</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Price</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Stock</th>
                        <th class="text-left px-4 py-3 font-semibold text-zinc-400">Status</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="product in products" :key="product.id"
                        class="border-t border-dark-300 hover:bg-dark-200 transition-colors">
                        <td class="px-4 py-3 text-zinc-200 font-medium">{{ product.name }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ product.sku }}</td>
                        <td class="px-4 py-3 text-zinc-200">${{ product.price.toFixed(2) }}</td>
                        <td class="px-4 py-3 text-zinc-400">{{ product.stock }}</td>
                        <td class="px-4 py-3">
                            <span class="px-2 py-1 rounded text-xs font-semibold" :class="product.status === 'active'
                                ? 'bg-lime-bg text-lime-main'
                                : 'bg-dark-300 text-zinc-400'">
                                {{ product.status }}
                            </span>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-else class="flex flex-col items-center justify-center py-16 px-4">
                <p class="text-zinc-300 font-semibold">No products</p>
                <p class="text-zinc-500 text-sm mt-1">Products you add will show up here.</p>
            </div>
        </div>
    </section>
</template>

<script setup lang="js">
definePageMeta({
    middleware: 'auth'
});

const products = ref([
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
    { id: 1, name: 'Wireless Mouse', sku: 'WM-1024', price: 24.99, stock: 142, status: 'active' },
    { id: 2, name: 'Mechanical Keyboard', sku: 'MK-2048', price: 89.99, stock: 58, status: 'active' },
    { id: 3, name: 'USB-C Hub', sku: 'UH-3072', price: 39.5, stock: 0, status: 'archived' },
    { id: 4, name: 'Monitor Stand', sku: 'MS-4096', price: 45.0, stock: 21, status: 'draft' },
    { id: 5, name: '27" 4K Monitor', sku: 'MN-5120', price: 329.99, stock: 12, status: 'active' },
]);
</script>