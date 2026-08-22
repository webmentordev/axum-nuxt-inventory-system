export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const slug = getRouterParam(event, 'slug');
    if (!slug) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Category slug is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/categories/public/${slug}`);
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Category does not exist'
                : (e.data?.message || 'Categorys fetch failed')
        });
    }
});