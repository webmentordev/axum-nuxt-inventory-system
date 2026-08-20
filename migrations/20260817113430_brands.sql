-- Add migration script here
CREATE TABLE
    brands (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(150) NOT NULL,
        slug VARCHAR(180) NOT NULL,
        description TEXT,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_brands_name UNIQUE (name),
        CONSTRAINT uq_brands_slug UNIQUE (slug)
    );

CREATE INDEX idx_brands_is_active ON brands (is_active);