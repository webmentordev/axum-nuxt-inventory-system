export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const body = await readBody(event);
    const allHeaders = getRequestHeaders(event);
    const orderUuid = getRouterParam(event, 'uuid');
    const itemUuid = getRouterParam(event, 'iuuid');
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
        const data = await $fetch(`${apiUrl}/api/admin/orders/${orderUuid}/items/${itemUuid}/status`, {
            method: "PATCH",
            headers: allHeaders,
            body: body
        });
        return data;
    } catch (e) {
        console.log(e)
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 409
                ? 'Order item status can not be changed'
                : (e.data?.message || 'Order item status update failed')
        });
    }
});