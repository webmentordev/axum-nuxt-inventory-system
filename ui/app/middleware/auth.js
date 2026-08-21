export default defineNuxtRouteMiddleware(async (to) => {
    const { getToken } = useAuthToken();
    const { getUser } = useAuthUser();
    const token = getToken();
    const user = getUser();
    if (!token || !user) {
        return await navigateTo('/login');
    }
    if (to.path.startsWith('/admin')) {
        if (!user.is_admin) {
            return await navigateTo('/');
        }
    }
});