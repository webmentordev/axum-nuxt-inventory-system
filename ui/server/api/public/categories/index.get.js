export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const allHeaders = getRequestHeaders(event);
    const query = getQuery(event);
    try {
        const data = await $fetch(`${apiUrl}/api/public/categories`, { headers: allHeaders, query: {
            sub_categories: query.sub_categories,
            is_featured: query.is_featured,
            with_uploads: query.with_uploads,
            limit: query.limit,
        } });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data.message || 'Categories fetch failed'
        });
    }
});