export const useCategories = () => {
  const { publicFetch } = usePublicFetch();
  const categories = useState('categories', () => []);
  const processing = useState('categoriesProcessing', () => true);
  const fetched = useState('categoriesFetched', () => false);
  const fetchCategories = async () => {
    if (fetched.value) {
      return categories.value;
    }
    processing.value = true;
    try {
      const data = await publicFetch('/api/public/categories?sub_categories=true&is_featured=true');
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