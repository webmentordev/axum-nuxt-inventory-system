export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const uuid = getRouterParam(event, 'uuid');
    const body = await readBody(event);
    const allHeaders = getRequestHeaders(event);
    if (!uuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Order id is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/admin/orders/${uuid}/items`, {
            method: "POST",
            headers: allHeaders,
            body: body
        });
        return data;
    } catch (e) {
        console.log(e)
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 409
                ? 'Order already exists'
                : (e.data?.message || 'Order creation failed')
        });
    }
});