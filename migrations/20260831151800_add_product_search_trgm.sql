-- Add migration script here
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX IF NOT EXISTS idx_products_name_trgm ON products USING gin (name gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_products_sku_trgm ON products USING gin (sku gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_products_model_trgm ON products USING gin (model gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_brands_name_trgm ON brands USING gin (name gin_trgm_ops);