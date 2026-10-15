-- Add migration script here
CREATE TABLE info_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    offer_title VARCHAR(150) NOT NULL,
    title VARCHAR(150) NOT NULL,
    description VARCHAR(500) NOT NULL,
    image_url VARCHAR(500),
    path_url VARCHAR(500) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_info_cards_title UNIQUE (title),
    CONSTRAINT ck_info_cards_sort_order CHECK (sort_order >= 0)
);

CREATE INDEX idx_info_cards_is_active ON info_cards (is_active);

CREATE INDEX idx_info_cards_sort_order ON info_cards (sort_order);