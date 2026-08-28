export const usePolicies = () => {
  const { publicFetch } = usePublicFetch();
  const cacheKey = 'policies';

  const policies = useState(cacheKey, () => []);
  const processing = useState(`${cacheKey}:processing`, () => true);
  const fetched = useState(`${cacheKey}:fetched`, () => false);

  const fetchPolicies = async () => {
    if (fetched.value) {
      return policies.value;
    }

    processing.value = true;
    try {
      const data = await publicFetch('/api/public/policies');
      if (data) {
        policies.value = data;
        fetched.value = true;
      }
    } catch (e) {
      throw createError({
        status: e.statusCode || 500,
        statusText: e.statusMessage || 'Something went wrong!',
        fatal: true
      });
    } finally {
      processing.value = false;
    }

    return policies.value;
  };

  return {
    policies,
    processing,
    fetchPolicies
  };
};