export default defineNuxtRouteMiddleware(async () => {
    const { getToken } = useAuthToken();
    const { getUser } = useAuthUser();
    const token = getToken();
    const user = getUser();

    if (token && user) {
        if (user.is_admin == true) {
            return await navigateTo('/admin/dashboard');
        }
        return await navigateTo('/');
    }
});