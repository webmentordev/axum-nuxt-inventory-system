function readRecentlyViewed() {
    if (!import.meta.client) return [];
    try {
        const raw = localStorage.getItem('recentlyViewed');
        return raw ? JSON.parse(raw) : [];
    } catch {
        return [];
    }
}

function writeRecentlyViewed(items) {
    if (!import.meta.client) return;
    localStorage.setItem('recentlyViewed', JSON.stringify(items));
}

export function useRecentlyViewed() {
    const recentlyViewedSlugs = useState('recentlyViewedSlugs', () => []);

    onMounted(() => {
        recentlyViewedSlugs.value = readRecentlyViewed();
    });

    watch(recentlyViewedSlugs, (items) => {
        writeRecentlyViewed(items);
    }, { deep: true });

    function addRecentlyViewed(slug) {
        if (!slug) return;

        const filtered = recentlyViewedSlugs.value.filter(s => s !== slug);
        filtered.unshift(slug);
        recentlyViewedSlugs.value = filtered.slice(0, 6);
    }

    function clearRecentlyViewed() {
        recentlyViewedSlugs.value = [];
    }

    const { publicFetch } = usePublicFetch();

    async function fetchRecentlyViewedProducts() {
        if (!recentlyViewedSlugs.value.length) return [];

        return await publicFetch('/api/public/products/recently-viewed', {
            method: 'POST',
            body: { slugs: recentlyViewedSlugs.value }
        });
    }

    return {
        recentlyViewedSlugs,
        addRecentlyViewed,
        clearRecentlyViewed,
        fetchRecentlyViewedProducts
    };
}