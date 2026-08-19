use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub subject: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateContact {
    pub name: String,
    pub email: Option<String>,
    pub subject: String,
    pub message: String,
}

pub async fn create_contact(
    State(state): State<AppState>,
    Json(payload): Json<CreateContact>,
) -> Result<(StatusCode, Json<Contact>), StatusCode> {
    let contact = sqlx::query_as!(
        Contact,
        r#"INSERT INTO contacts (name, email, subject, message)
           VALUES ($1, $2, $3, $4)
           RETURNING id, name, email, subject, message, created_at"#,
        payload.name,
        payload.email,
        payload.subject,
        payload.message
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(contact)))
}

pub async fn get_contacts(State(state): State<AppState>) -> Result<Json<Vec<Contact>>, StatusCode> {
    let contacts = sqlx::query_as!(
        Contact,
        r#"SELECT id, name, email, subject, message, created_at
           FROM contacts
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(contacts))
}

pub async fn delete_contact(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM contacts WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
