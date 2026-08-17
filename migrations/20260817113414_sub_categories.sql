-- Add migration script here
CREATE TABLE
    sub_categories (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        category_id UUID NOT NULL REFERENCES categories (id) ON DELETE CASCADE,
        name VARCHAR(100) NOT NULL,
        slug VARCHAR(120) NOT NULL,
        description TEXT,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_sub_categories_category_name UNIQUE (category_id, name),
        CONSTRAINT uq_sub_categories_slug UNIQUE (slug)
    );

CREATE INDEX idx_sub_categories_category_id ON sub_categories (category_id);

CREATE INDEX idx_sub_categories_is_active ON sub_categories (is_active);