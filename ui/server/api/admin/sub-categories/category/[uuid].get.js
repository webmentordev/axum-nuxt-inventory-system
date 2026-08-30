export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const uuid = getRouterParam(event, 'uuid');
    const allHeaders = getRequestHeaders(event);
    if (!uuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: "Sub category's category id is required" }
        });
    }
    try {
        const data = await $fetch(`${apiUrl}/api/admin/sub-categories/by-category/${uuid}`, { headers: allHeaders });
        return data;
    } catch (e) {
        console.log(e)
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data.message || "Sub category by category fetch failed"
        });
    }
});