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

    if (body.password !== body.confirm_password) {
        throw createError({
            statusCode: 422,
            statusMessage: 'Passwords do not match',
        });
    }

    try {
        const data = await $fetch(`${apiUrl}/api/public/users/register`, {
            method: "POST",
            body: {
                name: body.name,
                email: body.email,
                password: body.password
            }
        });
        return data;
    } catch (e) {
        const status = e.response?.status || 500;
        throw createError({
            statusCode: status,
            statusMessage: status === 409
                ? 'An account with this email already exists.'
                : status === 400
                    ? 'Please check your details. Password must be 8 to 128 characters.'
                    : (e.data?.message || 'Account creation failed')
        });
    }
});