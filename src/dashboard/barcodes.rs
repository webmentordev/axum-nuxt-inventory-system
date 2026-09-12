use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "barcode_type", rename_all = "lowercase")]
pub enum BarcodeType {
    Code128,
    Ean13,
    #[sqlx(rename = "upc_a")]
    UpcA,
    Qr,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Barcode {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub code: String,
    #[sqlx(rename = "type")]
    pub barcode_type: BarcodeType,
    pub is_sold: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBarcode {
    pub product_id: Option<Uuid>,
    pub code: String,
    pub barcode_type: BarcodeType,
}

#[derive(Debug, Deserialize)]
pub struct BulkBarcodeEntry {
    pub code: String,
    pub barcode_type: BarcodeType,
}

#[derive(Debug, Deserialize)]
pub struct CreateBarcodesBulk {
    pub product_id: Option<Uuid>,
    pub barcodes: Vec<BulkBarcodeEntry>,
}

#[derive(Debug, Serialize)]
pub struct CreateBarcodesBulkResult {
    pub created: Vec<Barcode>,
    pub skipped: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBarcode {
    pub product_id: Option<Uuid>,
    pub is_sold: Option<bool>,
}

pub async fn get_barcodes(State(state): State<AppState>) -> Result<Json<Vec<Barcode>>, StatusCode> {
    let barcodes = sqlx::query_as!(
        Barcode,
        r#"SELECT id, product_id, code, type as "barcode_type: BarcodeType", is_sold, created_at
           FROM barcodes
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(barcodes))
}

pub async fn create_barcode(
    State(state): State<AppState>,
    Json(payload): Json<CreateBarcode>,
) -> Result<(StatusCode, Json<Barcode>), StatusCode> {
    let code = payload.code.trim();
    if code.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let barcode = sqlx::query_as!(
        Barcode,
        r#"INSERT INTO barcodes (product_id, code, type)
           VALUES ($1, $2, $3)
           RETURNING id, product_id, code, type as "barcode_type: BarcodeType", is_sold, created_at"#,
        payload.product_id,
        code,
        payload.barcode_type as BarcodeType
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
            StatusCode::BAD_REQUEST
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::CREATED, Json(barcode)))
}

pub async fn create_barcodes_bulk(
    State(state): State<AppState>,
    Json(payload): Json<CreateBarcodesBulk>,
) -> Result<(StatusCode, Json<CreateBarcodesBulkResult>), StatusCode> {
    if payload.barcodes.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut seen = std::collections::HashSet::new();
    let mut created = Vec::new();
    let mut skipped = Vec::new();

    for entry in &payload.barcodes {
        let code = entry.code.trim();
        if code.is_empty() {
            continue;
        }
        if !seen.insert(code.to_string()) {
            skipped.push(code.to_string());
            continue;
        }

        let attempt = sqlx::query_as!(
            Barcode,
            r#"INSERT INTO barcodes (product_id, code, type)
               VALUES ($1, $2, $3)
               RETURNING id, product_id, code, type as "barcode_type: BarcodeType", is_sold, created_at"#,
            payload.product_id,
            code,
            entry.barcode_type as BarcodeType
        )
        .fetch_one(&mut *tx)
        .await;

        match attempt {
            Ok(b) => created.push(b),
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                skipped.push(code.to_string());
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateBarcodesBulkResult { created, skipped }),
    ))
}

pub async fn get_barcode(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Barcode>, StatusCode> {
    let barcode = sqlx::query_as!(
        Barcode,
        r#"SELECT id, product_id, code, type as "barcode_type: BarcodeType", is_sold, created_at
           FROM barcodes
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(barcode))
}

pub async fn update_barcode(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateBarcode>,
) -> Result<Json<Barcode>, StatusCode> {
    let barcode = sqlx::query_as!(
        Barcode,
        r#"UPDATE barcodes
           SET product_id = COALESCE($1, product_id),
               is_sold = COALESCE($2, is_sold)
           WHERE id = $3
           RETURNING id, product_id, code, type as "barcode_type: BarcodeType", is_sold, created_at"#,
        payload.product_id,
        payload.is_sold,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(barcode))
}

pub async fn delete_barcode(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM barcodes WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
