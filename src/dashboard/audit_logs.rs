use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub action: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub status: String,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

pub async fn get_audit_logs(
    State(state): State<AppState>,
    Query(params): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLog>>, StatusCode> {
    let limit = params.limit.unwrap_or(50).clamp(1, 200);
    let offset = params.offset.unwrap_or(0).max(0);

    let logs = sqlx::query_as!(
        AuditLog,
        r#"SELECT id, user_id, action, entity_type, entity_id, status, details, created_at
           FROM audit_logs
           WHERE ($1::TEXT IS NULL OR entity_type = $1)
             AND ($2::UUID IS NULL OR entity_id = $2)
             AND ($3::UUID IS NULL OR user_id = $3)
             AND ($4::TEXT IS NULL OR action = $4)
           ORDER BY created_at DESC
           LIMIT $5 OFFSET $6"#,
        params.entity_type,
        params.entity_id,
        params.user_id,
        params.action,
        limit,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(logs))
}
