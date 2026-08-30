CREATE TABLE uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    product_id UUID REFERENCES products (id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories (id) ON DELETE CASCADE,
    sub_category_id UUID REFERENCES sub_categories (id) ON DELETE CASCADE,
    brand_id UUID REFERENCES brands (id) ON DELETE CASCADE,
    name VARCHAR(150) NOT NULL,
    file_path TEXT NOT NULL,
    file_type VARCHAR(20) NOT NULL DEFAULT 'image',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ck_uploads_single_assignment CHECK (
        (product_id IS NOT NULL)::int + (category_id IS NOT NULL)::int + (sub_category_id IS NOT NULL)::int + (brand_id IS NOT NULL)::int <= 1
    )
);

CREATE INDEX idx_uploads_product_id ON uploads (product_id);

CREATE INDEX idx_uploads_category_id ON uploads (category_id);

CREATE INDEX idx_uploads_sub_category_id ON uploads (sub_category_id);

CREATE INDEX idx_uploads_brand_id ON uploads (brand_id);

CREATE UNIQUE INDEX uq_uploads_category_id ON uploads (category_id)
WHERE
    category_id IS NOT NULL;

CREATE UNIQUE INDEX uq_uploads_sub_category_id ON uploads (sub_category_id)
WHERE
    sub_category_id IS NOT NULL;

CREATE UNIQUE INDEX uq_uploads_brand_id ON uploads (brand_id)
WHERE
    brand_id IS NOT NULL;