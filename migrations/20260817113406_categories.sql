-- Add migration script here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE
    categories (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(100) NOT NULL,
        slug VARCHAR(120) NOT NULL,
        description TEXT,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_categories_name UNIQUE (name),
        CONSTRAINT uq_categories_slug UNIQUE (slug)
    );

CREATE INDEX idx_categories_is_active ON categories (is_active);