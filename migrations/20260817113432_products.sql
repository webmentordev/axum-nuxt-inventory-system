-- Add migration script here
CREATE TABLE
    products (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        category_id UUID NOT NULL REFERENCES categories (id) ON DELETE RESTRICT,
        sub_category_id UUID REFERENCES sub_categories (id) ON DELETE SET NULL,
        name VARCHAR(150) NOT NULL,
        sku VARCHAR(50) NOT NULL,
        brand VARCHAR(100),
        model VARCHAR(100),
        description TEXT,
        -- solar-specific specs (nullable since not every product has all of these)
        power_rating_watts NUMERIC(10, 2),
        voltage_rating NUMERIC(10, 2),
        capacity_ah NUMERIC(10, 2),
        warranty_months SMALLINT,
        -- pricing
        cost_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
        selling_price NUMERIC(12, 2) NOT NULL DEFAULT 0,
        -- inventory / stock
        quantity_in_stock INTEGER NOT NULL DEFAULT 0,
        reorder_level INTEGER NOT NULL DEFAULT 0,
        unit VARCHAR(20) NOT NULL DEFAULT 'piece',
        image_url TEXT,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_products_sku UNIQUE (sku),
        CONSTRAINT ck_products_quantity_non_negative CHECK (quantity_in_stock >= 0),
        CONSTRAINT ck_products_prices_non_negative CHECK (
            cost_price >= 0
            AND selling_price >= 0
        )
    );

CREATE INDEX idx_products_category_id ON products (category_id);

CREATE INDEX idx_products_sub_category_id ON products (sub_category_id);

CREATE INDEX idx_products_is_active ON products (is_active);

CREATE INDEX idx_products_sku ON products (sku);