export const useCategories = ({ featured = false, withSubCategories = false, withUploads = false } = {}) => {
  const { publicFetch } = usePublicFetch();
  const cacheKey = `categories:featured:${featured}:sub:${withSubCategories}`;

  const categories = useState(cacheKey, () => []);
  const processing = useState(`${cacheKey}:processing`, () => true);
  const fetched = useState(`${cacheKey}:fetched`, () => false);

  const fetchCategories = async () => {
    if (fetched.value) {
      return categories.value;
    }

    processing.value = true;
    try {
      const params = new URLSearchParams({
        sub_categories: String(withSubCategories),
        is_featured: String(featured),
        with_uploads: String(withUploads),
      });

      const data = await publicFetch(`/api/public/categories?${params}`);
      if (data) {
        categories.value = data;
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

    return categories.value;
  };

  return {
    categories,
    processing,
    fetchCategories
  };
};