export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const uuid = getRouterParam(event, 'uuid');
    const authHeader = getRequestHeaders(event);
    if (!uuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'SEO id is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/admin/seo/${uuid}`, {headers: authHeader});
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'SEO does not exist'
                : (e.data?.message || 'SEO fetch failed')
        });
    }
});