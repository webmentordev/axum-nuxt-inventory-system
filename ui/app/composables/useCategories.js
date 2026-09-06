export const useCategories = ({ featured = false, withSubCategories = false, withUploads = false, limit = 6 } = {}) => {
  const { publicFetch } = usePublicFetch();
  const cacheKey = `categories:featured:${featured}:sub:${withSubCategories}`;

  const { data: categories, pending: processing, error, refresh } = useAsyncData(
    cacheKey,
    () => {
      const params = new URLSearchParams({
        sub_categories: String(withSubCategories),
        is_featured: String(featured),
        with_uploads: String(withUploads),
        limit: String(limit),
      });
      return publicFetch(`/api/public/categories?${params}`);
    },
    { default: () => [] }
  );

  return {
    categories,
    processing,
    error,
    fetchCategories: refresh,
  };
};