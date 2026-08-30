-- Add migration script here
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    category_id UUID REFERENCES categories (id) ON DELETE SET NULL,
    sub_category_id UUID REFERENCES sub_categories (id) ON DELETE SET NULL,
    brand_id UUID REFERENCES brands (id) ON DELETE SET NULL,
    name VARCHAR(150) NOT NULL,
    slug VARCHAR(180) NOT NULL,
    sku VARCHAR(50) NOT NULL,
    brand VARCHAR(100),
    model VARCHAR(100),
    description TEXT,
    content TEXT,
    product_type VARCHAR(20) NOT NULL DEFAULT 'other',
    power_rating_watts NUMERIC(10, 2),
    voltage_rating NUMERIC(10, 2),
    capacity_ah NUMERIC(10, 2),
    warranty_months SMALLINT,
    cost_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
    selling_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
    compare_at_selling_price NUMERIC(12, 2),
    per_watt_price NUMERIC(12, 2),
    shipping_cost NUMERIC(12, 2) NOT NULL DEFAULT 0,
    tax NUMERIC(12, 2) NOT NULL DEFAULT 0,
    kilowatt_hour NUMERIC(10, 2),
    quantity_in_stock INTEGER NOT NULL DEFAULT 0,
    reorder_level INTEGER NOT NULL DEFAULT 0,
    unit VARCHAR(20) NOT NULL DEFAULT 'piece',
    image_url TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_products_sku UNIQUE (sku),
    CONSTRAINT uq_products_slug UNIQUE (slug),
    CONSTRAINT ck_products_quantity_non_negative CHECK (quantity_in_stock >= 0),
    CONSTRAINT ck_products_prices_non_negative CHECK (
        cost_price >= 0
        AND selling_price >= 0
        AND shipping_cost >= 0
        AND tax >= 0
        AND (
            compare_at_selling_price IS NULL
            OR compare_at_selling_price >= 0
        )
        AND (
            per_watt_price IS NULL
            OR per_watt_price >= 0
        )
    ),
    CONSTRAINT ck_products_product_type CHECK (
        product_type IN ('solar', 'other')
    )
);

CREATE INDEX idx_products_category_id ON products (category_id);

CREATE INDEX idx_products_sub_category_id ON products (sub_category_id);

CREATE INDEX idx_products_is_active ON products (is_active);

CREATE INDEX idx_products_sku ON products (sku);

CREATE INDEX idx_products_slug ON products (slug);

CREATE INDEX idx_products_brand_id ON products (brand_id);

CREATE INDEX idx_products_product_type ON products (product_type);