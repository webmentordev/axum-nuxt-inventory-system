export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const query = getQuery(event);
    const headers = getRequestHeaders(event);
    try {
        return await $fetch(`${apiUrl}/api/public/products`, { headers, query });
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data?.message || 'Products fetch failed'
        });
    }
});