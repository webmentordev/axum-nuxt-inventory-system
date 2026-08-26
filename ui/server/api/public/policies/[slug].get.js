export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const slug = getRouterParam(event, 'slug');
    if (!slug) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Policy slug is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/public/policies/${slug}`);
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Policy does not exist'
                : (e.data?.message || 'Policy fetch failed')
        });
    }
});