export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const slug = getRouterParam(event, 'slug');
    if (!slug) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Brand slug is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/public/brands/${slug}`);
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Brand does not exist'
                : (e.data?.message || 'Brand fetch failed')
        });
    }
});