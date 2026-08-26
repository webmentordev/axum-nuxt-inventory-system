export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const uuid = getRouterParam(event, 'uuid');
    const body = await readBody(event);
    const authHeader = getRequestHeaders(event);
    if (!uuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Policy id is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/admin/policies/${uuid}`, {
            method: 'PATCH',
            headers: authHeader,
            body: body
        });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Policy does not exist'
                : (e.data?.message || 'Policy status update failed')
        });
    }
});