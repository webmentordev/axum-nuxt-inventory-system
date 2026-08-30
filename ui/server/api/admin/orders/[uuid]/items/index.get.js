export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const uuid = getRouterParam(event, 'uuid');
    const allHeaders = getRequestHeaders(event);
    if (!uuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Order id is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/admin/orders/${uuid}/items`, { headers: allHeaders });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data.message || 'Orders items fetch failed'
        });
    }
});