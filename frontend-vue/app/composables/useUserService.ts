export const useUserService = () => {
  const { apiCall } = useApi();

  const register = () => apiCall(`/user/register`, { method: 'POST' });

  return { register };
};
