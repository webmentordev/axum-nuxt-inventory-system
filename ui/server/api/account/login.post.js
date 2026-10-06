export default defineEventHandler(async (event) => {
    const apiUrl = useRuntimeConfig(event).apiUrl;
    const body = await readBody(event);

    if (!body?.email || !body?.password) {
        throw createError({
            statusCode: 422,
            statusMessage: 'Email and password are required.',
        });
    }

    try {
        const data = await $fetch(`${apiUrl}/api/public/users/login`, {
            method: "POST",
            body: {
                email: body.email,
                password: body.password
            }
        });
        return data;
    } catch (e) {
        const status = e.response?.status || 500;
        const code = e.data?.code;

        let statusMessage = e.data?.message || 'Account login failed';
        if (status === 401) {
            statusMessage = 'Invalid login credentials';
        } else if (status === 403 && !code) {
            statusMessage = 'Your account has been deactivated.';
        }

        throw createError({
            statusCode: status,
            statusMessage,
            data: { code }
        });
    }
});