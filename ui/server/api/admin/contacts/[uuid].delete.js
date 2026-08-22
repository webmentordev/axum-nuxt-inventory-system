export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const uuid = getRouterParam(event, 'uuid');
    const authHeader = getRequestHeaders(event);
    if (!uuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'Contact id is required' }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/admin/contacts/${uuid}`, {
            method: 'DELETE',
            headers: authHeader
        });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 404
                ? 'Contact does not exist'
                : (e.data?.message || 'Contact delete failed')
        });
    }
});