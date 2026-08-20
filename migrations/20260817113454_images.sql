-- Add migration script here
CREATE TABLE images (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    product_id UUID REFERENCES products (id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories (id) ON DELETE CASCADE,
    sub_category_id UUID REFERENCES sub_categories (id) ON DELETE CASCADE,
    brand_id UUID REFERENCES brands (id) ON DELETE CASCADE,
    name VARCHAR(150) NOT NULL,
    file_path TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ck_images_single_assignment CHECK (
        (product_id IS NOT NULL)::int + (category_id IS NOT NULL)::int + (sub_category_id IS NOT NULL)::int + (brand_id IS NOT NULL)::int <= 1
    )
);

CREATE INDEX idx_images_product_id ON images (product_id);
CREATE INDEX idx_images_category_id ON images (category_id);
CREATE INDEX idx_images_sub_category_id ON images (sub_category_id);
CREATE INDEX idx_images_brand_id ON images (brand_id);