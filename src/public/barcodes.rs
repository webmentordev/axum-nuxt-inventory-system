use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct VerifyBarcode {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyBarcodeResult {
    pub exists: bool,
}

pub async fn verify_barcode(
    State(state): State<AppState>,
    Json(payload): Json<VerifyBarcode>,
) -> Result<Json<VerifyBarcodeResult>, StatusCode> {
    let code = payload.code.trim();
    if code.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM barcodes WHERE code = $1) as "exists!""#,
        code
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(VerifyBarcodeResult { exists }))
}
