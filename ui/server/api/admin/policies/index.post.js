export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const body = await readBody(event);
    const allHeaders = getRequestHeaders(event);
    try {
        const data = await $fetch(`${apiUrl}/api/admin/policies`, {
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
                ? 'Policy already exists'
                : (e.data?.message || 'Policy creation failed')
        });
    }
});