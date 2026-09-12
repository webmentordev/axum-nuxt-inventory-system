export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const body = await readBody(event);
    const allHeaders = getRequestHeaders(event);
    try {
        const data = await $fetch(`${apiUrl}/api/admin/barcodes`, {
            method: "POST",
            headers: allHeaders,
            body: body
        });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.response?.status === 409
                ? 'Barcode already exists'
                : (e.data?.message || 'Barcode creation failed')
        });
    }
});