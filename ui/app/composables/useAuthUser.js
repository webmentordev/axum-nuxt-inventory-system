export const useAuthUser = () => {
    const user = useCookie('klm_slr_user', {
        maxAge: 60 * 60,
        sameSite: 'strict',
        secure: import.meta.client ? window.location.protocol === 'https:' : true,
        path: '/'
    });

    const setUser = (newUser) => {
        user.value = newUser;
    };
    const removeUser = () => {
        user.value = null;
    };
    const getUser = () => {
        return user.value;
    };
    return {
        user,
        setUser,
        removeUser,
        getUser
    };
};