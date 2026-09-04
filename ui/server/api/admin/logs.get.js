export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const query = getQuery(event);
    const allHeaders = getRequestHeaders(event);
    try {
        const data = await $fetch(`${apiUrl}/api/admin/audit-logs`, { headers: allHeaders, query });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data.message || 'Audit logs fetch failed'
        });
    }
});