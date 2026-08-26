export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const slug = getRouterParam(event, 'slug');
    if (!slug) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Product slug is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/public/products/${slug}`);
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Product does not exist'
                : (e.data?.message || 'Product fetch failed')
        });
    }
});