-- Add migration script here
ALTER TABLE users
ADD COLUMN email_verified_at TIMESTAMPTZ,
ADD COLUMN token_version INTEGER NOT NULL DEFAULT 0;

CREATE TABLE user_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    purpose VARCHAR(30) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_user_tokens_token_hash UNIQUE (token_hash),
    CONSTRAINT chk_user_tokens_purpose CHECK (
        purpose IN (
            'email_verification',
            'password_reset'
        )
    )
);

CREATE INDEX idx_user_tokens_user_purpose ON user_tokens (user_id, purpose);

CREATE INDEX idx_user_tokens_expires_at ON user_tokens (expires_at);

CREATE UNIQUE INDEX uq_users_email_lower ON users (LOWER(email));