export const useAuthFetch = () => {
    const { getToken } = useAuthToken();
    const authFetch = async (url, options = {}) => {
        const token = getToken();
        const isFormData = options.body instanceof FormData;

        const defaultOptions = {
            ...options,
            headers: {
                'Authorization': token ? `Bearer ${token}` : '',
                'Accept': 'application/json',
                ...(isFormData ? {} : { 'Content-Type': 'application/json' }),
                ...options.headers
            }
        };
        try {
            return await $fetch(url, defaultOptions);
        } catch (error) {
            if (error.status === 401 || error.status === 403) {
                const { removeToken } = useAuthToken();
                removeToken();
                await navigateTo('/login');
            }
            throw error;
        }
    };
    return { authFetch };
};