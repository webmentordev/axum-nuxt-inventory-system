export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const allHeaders = getRequestHeaders(event);
    const query = getQuery(event);
    try {
        const data = await $fetch(`${apiUrl}/api/admin/orders/barcodes/lookup`, { headers: allHeaders, query: query });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data.message || 'Orders fetch failed'
        });
    }
});