export const useApi = () => {
  const { getToken } = useAuth();
  const config = useRuntimeConfig();

  const apiCall = async <T>(
    path: string,
    options: Parameters<typeof $fetch>[1] = {},
  ): Promise<T> => {
    // @ts-ignore
    const token = await getToken.value();
    return $fetch<T>(`${config.public.apiBase}${path}`, {
      ...options,
      headers: {
        ...options.headers,
        Authorization: `Bearer ${token}`,
      },
    });
  };

  return { apiCall: apiCall };
};
