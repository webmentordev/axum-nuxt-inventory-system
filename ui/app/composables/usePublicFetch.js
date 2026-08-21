export const usePublicFetch = () => {
    const publicFetch = async (url, options = {}) => {
        const defaultOptions = {
            ...options,
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                ...options.headers
            }
        };
        try {
            return await $fetch(url, defaultOptions);
        } catch (error) {
            if (error.status === 401) {
                await navigateTo('/login');
            }
            throw error;
        }
    };
    return { publicFetch };
};