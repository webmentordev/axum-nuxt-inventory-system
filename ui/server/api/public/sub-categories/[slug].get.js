export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const query = getQuery(event);
    const slug = getRouterParam(event, 'slug');
    if (!slug) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Sub category slug is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/public/sub-categories/${slug}`, { query });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Sub category does not exist'
                : (e.data?.message || 'Sub category fetch failed')
        });
    }
});