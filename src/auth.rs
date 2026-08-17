use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)] // never send the hash back in API responses
    pub password_hash: String,

    pub is_admin: bool,
    pub is_active: bool,

    pub last_login_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Public-safe version to return from API responses (no password_hash at all)
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct UserPublic {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
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
        }
    }
}

// ---------- Request DTOs ----------

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String, // plaintext in, hashed before insert — never stored raw
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

// For JWT claims (jsonwebtoken)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user id
    pub is_admin: bool,
    pub exp: i64, // expiry timestamp
}
