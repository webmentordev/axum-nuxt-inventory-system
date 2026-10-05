use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Duration;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::tokens::{
    EMAIL_VERIFICATION, PASSWORD_RESET, consume_token, issue_token, recently_issued,
};

pub enum AccountError {
    Invalid(&'static str),
    InvalidToken,
    Internal,
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AccountError::Invalid(message) => (StatusCode::BAD_REQUEST, message.to_string()),
            AccountError::InvalidToken => (
                StatusCode::BAD_REQUEST,
                "This link is invalid or has expired.".to_string(),
            ),
            AccountError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.".to_string(),
            ),
        };

        (status, Json(json!({ "message": message }))).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct EmailRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

fn generic_response() -> Json<Value> {
    Json(json!({
        "message": "If that email is registered, we've sent instructions to it."
    }))
}

pub fn queue_verification_email(state: AppState, user_id: Uuid, email: String) {
    tokio::spawn(async move {
        if let Err(err) = send_verification_email(&state, user_id, &email).await {
            eprintln!("verification email failed: {err}");
        }
    });
}

async fn send_verification_email(
    state: &AppState,
    user_id: Uuid,
    email: &str,
) -> anyhow::Result<()> {
    if recently_issued(&state.db, user_id, EMAIL_VERIFICATION).await? {
        return Ok(());
    }

    let token = issue_token(&state.db, user_id, EMAIL_VERIFICATION, Duration::hours(24)).await?;
    let link = format!("{}/verify-email?token={}", state.mailer.frontend_url, token);

    state
        .mailer
        .send(
            email,
            "Verify your email address",
            format!(
                "Open this link to verify your email address:\n\n{link}\n\nThis link expires in 24 hours."
            ),
        )
        .await
}

async fn send_password_reset(state: &AppState, email: &str) -> anyhow::Result<()> {
    let user = sqlx::query!(
        "SELECT id, is_active FROM users WHERE LOWER(email) = $1",
        email
    )
    .fetch_optional(&state.db)
    .await?;

    let Some(user) = user else {
        return Ok(());
    };

    if !user.is_active {
        return Ok(());
    }

    if recently_issued(&state.db, user.id, PASSWORD_RESET).await? {
        return Ok(());
    }

    let token = issue_token(&state.db, user.id, PASSWORD_RESET, Duration::minutes(30)).await?;
    let link = format!(
        "{}/reset-password?token={}",
        state.mailer.frontend_url, token
    );

    state
        .mailer
        .send(
            email,
            "Reset your password",
            format!(
                "Open this link to choose a new password:\n\n{link}\n\nThis link expires in 30 minutes. If you didn't request this, you can ignore this email."
            ),
        )
        .await
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<EmailRequest>,
) -> Result<Json<Value>, AccountError> {
    let email = normalize_email(&payload.email);

    if email.is_empty() || !email.contains('@') {
        return Err(AccountError::Invalid("Please enter a valid email address."));
    }

    tokio::spawn(async move {
        if let Err(err) = send_password_reset(&state, &email).await {
            eprintln!("password reset email failed: {err}");
        }
    });

    Ok(generic_response())
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<Value>, AccountError> {
    if payload.password.len() < 8 {
        return Err(AccountError::Invalid(
            "Password must be at least 8 characters.",
        ));
    }
    if payload.password.len() > 128 {
        return Err(AccountError::Invalid(
            "Password must be at most 128 characters.",
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| AccountError::Internal)?
        .to_string();

    let mut tx = state.db.begin().await.map_err(|_| AccountError::Internal)?;

    let user_id = consume_token(&mut tx, payload.token.trim(), PASSWORD_RESET)
        .await
        .map_err(|_| AccountError::Internal)?
        .ok_or(AccountError::InvalidToken)?;

    let email = sqlx::query_scalar!(
        r#"UPDATE users
           SET password_hash = $1,
               token_version = token_version + 1,
               email_verified_at = COALESCE(email_verified_at, NOW()),
               updated_at = NOW()
           WHERE id = $2
           RETURNING email as "email!""#,
        password_hash,
        user_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| AccountError::Internal)?;

    sqlx::query!(
        "DELETE FROM user_tokens WHERE user_id = $1 AND purpose = $2 AND used_at IS NULL",
        user_id,
        PASSWORD_RESET
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| AccountError::Internal)?;

    tx.commit().await.map_err(|_| AccountError::Internal)?;

    let mailer = state.mailer.clone();
    tokio::spawn(async move {
        let _ = mailer
            .send(
                &email,
                "Your password was changed",
                "Your password was just changed. If this wasn't you, contact support immediately."
                    .to_string(),
            )
            .await;
    });

    Ok(Json(json!({ "message": "Your password has been reset." })))
}

pub async fn verify_email(
    State(state): State<AppState>,
    Json(payload): Json<TokenRequest>,
) -> Result<Json<Value>, AccountError> {
    let mut tx = state.db.begin().await.map_err(|_| AccountError::Internal)?;

    let user_id = consume_token(&mut tx, payload.token.trim(), EMAIL_VERIFICATION)
        .await
        .map_err(|_| AccountError::Internal)?
        .ok_or(AccountError::InvalidToken)?;

    sqlx::query!(
        "UPDATE users SET email_verified_at = COALESCE(email_verified_at, NOW()), updated_at = NOW() WHERE id = $1",
        user_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| AccountError::Internal)?;

    tx.commit().await.map_err(|_| AccountError::Internal)?;

    Ok(Json(json!({ "message": "Your email has been verified." })))
}

pub async fn resend_verification(
    State(state): State<AppState>,
    Json(payload): Json<EmailRequest>,
) -> Result<Json<Value>, AccountError> {
    let email = normalize_email(&payload.email);

    if email.is_empty() || !email.contains('@') {
        return Err(AccountError::Invalid("Please enter a valid email address."));
    }

    tokio::spawn(async move {
        let user = sqlx::query!(
            "SELECT id, email FROM users WHERE LOWER(email) = $1 AND is_active = TRUE AND email_verified_at IS NULL",
            email
        )
        .fetch_optional(&state.db)
        .await;

        if let Ok(Some(user)) = user {
            if let Err(err) = send_verification_email(&state, user.id, &user.email).await {
                eprintln!("verification email failed: {err}");
            }
        }
    });

    Ok(generic_response())
}
