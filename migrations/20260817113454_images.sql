-- Add migration script here
CREATE TABLE
    images (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        product_id UUID REFERENCES products (id) ON DELETE CASCADE,
        name VARCHAR(150) NOT NULL,
        file_path TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE INDEX idx_images_product_id ON images (product_id);