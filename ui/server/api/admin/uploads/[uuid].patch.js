export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const parts = await readMultipartFormData(event);
    const authHeader = getRequestHeader(event, 'authorization');
    const uuid = getRouterParam(event, 'uuid');
    if (!uuid) {
        throw createError({
            statusCode: 400,
            statusMessage: 'Bad Request',
            data: { message: 'User id is required' }
        });
    }
    const formData = new FormData();
    for (const part of parts) {
        if (part.filename) {
            formData.append(
                part.name,
                new Blob([part.data], { type: part.type }),
                part.filename
            );
        } else {
            formData.append(part.name, part.data.toString());
        }
    }

    try {
        const data = await $fetch(`${apiUrl}/api/admin/uploads/${uuid}`, {
            method: "PATCH",
            headers: {
                ...(authHeader ? { Authorization: authHeader } : {})
            },
            body: formData
        });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data?.message || 'File update failed'
        });
    }
});