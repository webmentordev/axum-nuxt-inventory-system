-- Add migration script here
CREATE TABLE product_specifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    group_name VARCHAR(100),
    key VARCHAR(100) NOT NULL,
    value TEXT NOT NULL,
    unit VARCHAR(30),
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_highlighted BOOLEAN NOT NULL DEFAULT FALSE,
    is_filterable BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_product_specifications_product_key UNIQUE (product_id, group_name, key),
    CONSTRAINT ck_product_specifications_sort_order_non_negative CHECK (sort_order >= 0)
);

CREATE INDEX idx_product_specifications_product_id ON product_specifications (product_id);

CREATE INDEX idx_product_specifications_key ON product_specifications (key);

CREATE INDEX idx_product_specifications_is_active ON product_specifications (is_active);

CREATE INDEX idx_product_specifications_product_sort ON product_specifications (product_id, sort_order);

CREATE INDEX idx_product_specifications_filterable ON product_specifications (key, value)
WHERE
    is_filterable = TRUE
    AND is_active = TRUE;