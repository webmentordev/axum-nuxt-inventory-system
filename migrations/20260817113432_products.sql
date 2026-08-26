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
    panel_type VARCHAR(50),
    cell_type VARCHAR(50),
    number_of_cells SMALLINT,
    efficiency_percentage NUMERIC(5, 2),
    max_system_voltage NUMERIC(10, 2),
    open_circuit_voltage NUMERIC(10, 2),
    short_circuit_current NUMERIC(10, 2),
    max_power_voltage NUMERIC(10, 2),
    max_power_current NUMERIC(10, 2),
    temperature_coefficient NUMERIC(6, 3),
    frame_material VARCHAR(50),
    glass_type VARCHAR(50),
    length_mm NUMERIC(10, 2),
    width_mm NUMERIC(10, 2),
    thickness_mm NUMERIC(10, 2),
    weight_kg NUMERIC(10, 2),
    cost_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
    compare_at_cost_price NUMERIC(12, 2),
    selling_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
    compare_at_selling_price NUMERIC(12, 2),
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
        AND (
            compare_at_cost_price IS NULL
            OR compare_at_cost_price >= 0
        )
        AND (
            compare_at_selling_price IS NULL
            OR compare_at_selling_price >= 0
        )
    ),
    CONSTRAINT ck_products_product_type CHECK (
        product_type IN ('solar', 'other')
    ),
    CONSTRAINT ck_products_efficiency_range CHECK (
        efficiency_percentage IS NULL
        OR (
            efficiency_percentage >= 0
            AND efficiency_percentage <= 100
        )
    )
);

CREATE INDEX idx_products_category_id ON products (category_id);

CREATE INDEX idx_products_sub_category_id ON products (sub_category_id);

CREATE INDEX idx_products_is_active ON products (is_active);

CREATE INDEX idx_products_sku ON products (sku);

CREATE INDEX idx_products_slug ON products (slug);

CREATE INDEX idx_products_brand_id ON products (brand_id);

CREATE INDEX idx_products_product_type ON products (product_type);