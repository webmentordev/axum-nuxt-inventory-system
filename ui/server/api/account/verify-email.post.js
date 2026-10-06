export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const body = await readBody(event);

    if (!body?.token || typeof body.token !== 'string') {
        throw createError({
            statusCode: 422,
            statusMessage: 'Verification token not provided.',
        });
    }

    try {
        const data = await $fetch(`${apiUrl}/api/public/account/verify-email`, {
            method: "POST",
            body: { token: body.token }
        });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data?.message || 'Email verification failed'
        });
    }
});