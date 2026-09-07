export default defineEventHandler(async (event) => {
    // Auth is required.
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const allHeaders = getRequestHeaders(event);
    try {
        const data = await $fetch(`${apiUrl}/api/public/orders/user-orders`, { headers: allHeaders });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data.message || 'Contacts fetch failed'
        });
    }
});