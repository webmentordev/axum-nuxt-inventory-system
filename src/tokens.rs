use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

pub const EMAIL_VERIFICATION: &str = "email_verification";
pub const PASSWORD_RESET: &str = "password_reset";

pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

pub async fn issue_token(
    db: &PgPool,
    user_id: Uuid,
    purpose: &str,
    ttl: Duration,
) -> Result<String, sqlx::Error> {
    let token = generate_token();
    let token_hash = hash_token(&token);
    let expires_at = Utc::now() + ttl;

    let mut tx = db.begin().await?;

    sqlx::query!(
        "DELETE FROM user_tokens WHERE user_id = $1 AND purpose = $2 AND used_at IS NULL",
        user_id,
        purpose
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "INSERT INTO user_tokens (user_id, token_hash, purpose, expires_at) VALUES ($1, $2, $3, $4)",
        user_id,
        token_hash,
        purpose,
        expires_at
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(token)
}

pub async fn consume_token(
    conn: &mut PgConnection,
    token: &str,
    purpose: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let token_hash = hash_token(token);

    sqlx::query_scalar!(
        r#"UPDATE user_tokens
           SET used_at = NOW()
           WHERE token_hash = $1
             AND purpose = $2
             AND used_at IS NULL
             AND expires_at > NOW()
           RETURNING user_id as "user_id!""#,
        token_hash,
        purpose
    )
    .fetch_optional(&mut *conn)
    .await
}

pub async fn recently_issued(
    db: &PgPool,
    user_id: Uuid,
    purpose: &str,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(
               SELECT 1 FROM user_tokens
               WHERE user_id = $1
                 AND purpose = $2
                 AND used_at IS NULL
                 AND created_at > NOW() - INTERVAL '60 seconds'
           ) as "exists!""#,
        user_id,
        purpose
    )
    .fetch_one(db)
    .await
}
