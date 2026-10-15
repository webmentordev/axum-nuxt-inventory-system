use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize)]
pub struct PublicInfoCard {
    pub id: Uuid,
    pub offer_title: String,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub path_url: String,
}

pub async fn get_public_info_cards(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicInfoCard>>, StatusCode> {
    let cards = sqlx::query_as!(
        PublicInfoCard,
        r#"SELECT id, offer_title, title, description, image_url, path_url
           FROM info_cards
           WHERE is_active = TRUE
           ORDER BY sort_order ASC, created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(cards))
}
