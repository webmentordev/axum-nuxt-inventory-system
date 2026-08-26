-- Add migration script here
CREATE TABLE policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name VARCHAR(150) NOT NULL,
    slug VARCHAR(180) NOT NULL,
    seo_title VARCHAR(255),
    seo_description VARCHAR(255),
    content TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_policies_slug UNIQUE (slug)
);

CREATE INDEX idx_policies_is_active ON policies (is_active);

CREATE INDEX idx_policies_slug ON policies (slug);

CREATE INDEX idx_policies_sort_order ON policies (sort_order);