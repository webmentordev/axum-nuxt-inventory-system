export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const authHeader = getRequestHeader(event, 'authorization');
    const contentType = getRequestHeader(event, 'content-type');
    const contentLength = getRequestHeader(event, 'content-length');

    let response;
    try {
        response = await fetch(`${apiUrl}/api/admin/uploads/tmp`, {
            method: 'POST',
            headers: {
                ...(authHeader ? { Authorization: authHeader } : {}),
                ...(contentType ? { 'Content-Type': contentType } : {}),
                ...(contentLength ? { 'Content-Length': contentLength } : {})
            },
            body: event.node.req,
            duplex: 'half'
        });
    } catch (e) {
        throw createError({
            statusCode: 500,
            statusMessage: 'File upload failed'
        });
    }

    const data = await response.json().catch(() => null);

    if (!response.ok) {
        throw createError({
            statusCode: response.status,
            statusMessage: data?.message || 'File upload failed'
        });
    }

    return data;
});