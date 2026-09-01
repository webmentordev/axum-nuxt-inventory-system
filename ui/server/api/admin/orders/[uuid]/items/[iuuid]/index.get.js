export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const orderUuid = getRouterParam(event, 'uuid');
    const itemUuid = getRouterParam(event, 'iuuid');
    const authHeader = getRequestHeaders(event);
    if (!orderUuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Order id is required' }
        });
    }
    if (!itemUuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Item id is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/admin/orders/${orderUuid}/items/${itemUuid}`, {headers: authHeader});
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Item does not exist'
                : (e.data?.message || 'Item fetch failed')
        });
    }
});