use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::public::account::queue_verification_email;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub is_admin: bool,
    pub is_active: bool,

    pub last_login_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct UserPublic {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            email: u.email,
            is_admin: u.is_admin,
            is_active: u.is_active,
            last_login_at: u.last_login_at,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub is_admin: bool,
    #[serde(default)]
    pub ver: i32,
    pub exp: i64,
}

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserPublic>>, StatusCode> {
    let users = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password_hash, is_admin, is_active, last_login_at, created_at, updated_at
           FROM users
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users = users.into_iter().map(UserPublic::from).collect();

    Ok(Json(users))
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<UserPublic>), StatusCode> {
    let name = payload.name.trim().to_string();
    let email = payload.email.trim().to_lowercase();

    if name.is_empty()
        || !email.contains('@')
        || payload.password.len() < 8
        || payload.password.len() > 128
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let is_admin = existing_count == 0;

    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (name, email, password_hash, is_admin)
           VALUES ($1, $2, $3, $4)
           RETURNING id, name, email, password_hash, is_admin, is_active, last_login_at, created_at, updated_at"#,
        name,
        email,
        password_hash,
        is_admin
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    queue_verification_email(state.clone(), user.id, user.email.clone());

    Ok((StatusCode::CREATED, Json(UserPublic::from(user))))
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let email = payload.email.trim().to_lowercase();

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password_hash, is_admin, is_active, last_login_at, created_at, updated_at
           FROM users
           WHERE LOWER(email) = $1"#,
        email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    if !user.is_active {
        return Err(StatusCode::FORBIDDEN);
    }

    let parsed_hash =
        PasswordHash::new(&user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let auth_state = sqlx::query!(
        "SELECT token_version, email_verified_at FROM users WHERE id = $1",
        user.id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if auth_state.email_verified_at.is_none() {
        return Ok((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "code": "email_not_verified",
                "message": "Please verify your email address before logging in."
            })),
        ));
    }

    sqlx::query!(
        "UPDATE users SET last_login_at = NOW() WHERE id = $1",
        user.id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let exp = (Utc::now() + Duration::hours(24 * 30)).timestamp();
    let claims = Claims {
        sub: user.id,
        is_admin: user.is_admin,
        ver: auth_state.token_version,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "token": token,
            "user": UserPublic::from(user)
        })),
    ))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<UserPublic>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password_hash, is_admin, is_active, last_login_at, created_at, updated_at
           FROM users
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserPublic::from(user)))
}

async fn get_first_admin_id(state: &AppState) -> Result<Option<Uuid>, StatusCode> {
    let id = sqlx::query_scalar!(
        r#"SELECT id FROM users
           WHERE is_admin = TRUE
           ORDER BY created_at ASC
           LIMIT 1"#
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(id)
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<UserPublic>, StatusCode> {
    let first_admin_id = get_first_admin_id(&state).await?;

    if first_admin_id == Some(uuid) && (payload.is_admin.is_some() || payload.is_active.is_some()) {
        return Err(StatusCode::FORBIDDEN);
    }

    let name = payload.name.as_deref().map(|n| n.trim().to_string());
    let email = payload.email.as_deref().map(|e| e.trim().to_lowercase());

    if name.as_deref().is_some_and(|n| n.is_empty())
        || email.as_deref().is_some_and(|e| !e.contains('@'))
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = sqlx::query_as!(
        User,
        r#"UPDATE users
           SET name = COALESCE($1, name),
               email_verified_at = CASE
                   WHEN $2::text IS NOT NULL AND $2::text <> email THEN NULL
                   ELSE email_verified_at
               END,
               email = COALESCE($2, email),
               is_admin = COALESCE($3, is_admin),
               is_active = COALESCE($4, is_active),
               updated_at = NOW()
           WHERE id = $5
           RETURNING id, name, email, password_hash, is_admin, is_active, last_login_at, created_at, updated_at"#,
        name,
        email,
        payload.is_admin,
        payload.is_active,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserPublic::from(user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    if claims.sub == uuid {
        return Err(StatusCode::FORBIDDEN);
    }

    let first_admin_id = get_first_admin_id(&state).await?;

    if first_admin_id == Some(uuid) {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query!("DELETE FROM users WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
