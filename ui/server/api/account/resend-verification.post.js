export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const body = await readBody(event);

    if (!body?.ct_token) {
        throw createError({
            statusCode: 422,
            statusMessage: 'Token not provided.',
        });
    }

    const result = await verifyTurnstileToken(body.ct_token);
    if (!result?.success) {
        throw createError({ statusCode: 403, statusMessage: 'Token check failed' });
    }

    try {
        const data = await $fetch(`${apiUrl}/api/public/account/resend-verification`, {
            method: "POST",
            body: { email: body.email }
        });
        return data;
    } catch (e) {
        throw createError({
            statusCode: e.response?.status || 500,
            statusMessage: e.data?.message || 'Verification request failed'
        });
    }
});