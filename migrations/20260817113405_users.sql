-- Add migration script here
CREATE TABLE
    users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name VARCHAR(150) NOT NULL,
        email VARCHAR(150) NOT NULL,
        password_hash TEXT NOT NULL,
        is_admin BOOLEAN NOT NULL DEFAULT FALSE,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        last_login_at TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_users_email UNIQUE (email)
    );

CREATE INDEX idx_users_email ON users (email);

CREATE INDEX idx_users_is_active ON users (is_active);