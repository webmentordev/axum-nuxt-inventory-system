export function useCart() {
    const cartItems = useState('cartItems', () => []);

    const cartCount = computed(() =>
        cartItems.value.reduce((sum, item) => sum + item.quantity, 0)
    );

    function addToCart(product, quantity = 1) {
        const existing = cartItems.value.find(item => item.slug === product.slug);

        if (existing) {
            existing.quantity += quantity;
            existing.unit_price = Number(product.selling_price) || 0;
        } else {
            cartItems.value.push({
                slug: product.slug,
                sku: product.sku,
                name: product.name,
                image_url: product.image_url,
                unit_price: Number(product.selling_price) || 0,
                quantity
            });
        }
    }

    function updateQuantity(slug, quantity) {
        const item = cartItems.value.find(item => item.slug === slug);
        if (!item) return;

        if (quantity <= 0) {
            removeFromCart(slug);
            return;
        }

        item.quantity = quantity;
    }

    function removeFromCart(slug) {
        cartItems.value = cartItems.value.filter(item => item.slug !== slug);
    }

    function clearCart() {
        cartItems.value = [];
    }

    function isInCart(slug) {
        return cartItems.value.some(item => item.slug === slug);
    }

    return {
        cartItems,
        cartCount,
        addToCart,
        updateQuantity,
        removeFromCart,
        clearCart,
        isInCart
    };
}