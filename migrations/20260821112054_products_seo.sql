-- Add migration script here
CREATE TABLE
    products_seo (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
        meta_title VARCHAR(70),
        meta_description VARCHAR(160),
        meta_keywords TEXT,
        og_title VARCHAR(70),
        og_description VARCHAR(200),
        og_image_url TEXT,
        canonical_url TEXT,
        focus_keyword VARCHAR(150),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_products_seo_product_id UNIQUE (product_id)
    );

CREATE INDEX idx_products_seo_product_id ON products_seo (product_id);