use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path as FsPath;
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use crate::AppState;
use crate::utils::slugify;

const UPLOAD_DIR: &str = "uploads/files";

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Upload {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub sub_category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub name: String,
    pub file_path: String,
    pub file_type: String,
    pub created_at: DateTime<Utc>,
}

pub trait WithFullUrl {
    fn with_full_url(self) -> Self;
}

impl WithFullUrl for Upload {
    fn with_full_url(mut self) -> Self {
        let mut domain =
            std::env::var("DOMAIN").unwrap_or_else(|_| "http://127.0.0.1:7765".to_string());
        if domain.trim().is_empty() {
            domain = "http://127.0.0.1:7765".to_string();
        }
        let domain = domain.trim_end_matches('/');
        self.file_path = format!("{domain}/{}", self.file_path);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MiniProduct {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MiniCategory {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MiniSubCategory {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MiniBrand {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadWithDetails {
    pub id: Uuid,
    pub product: Option<MiniProduct>,
    pub category: Option<MiniCategory>,
    pub sub_category: Option<MiniSubCategory>,
    pub brand: Option<MiniBrand>,
    pub name: String,
    pub file_path: String,
    pub file_type: String,
    pub created_at: DateTime<Utc>,
}

async fn attach_details(
    state: &AppState,
    uploads: Vec<Upload>,
) -> Result<Vec<UploadWithDetails>, StatusCode> {
    let product_ids: Vec<Uuid> = uploads.iter().filter_map(|i| i.product_id).collect();
    let category_ids: Vec<Uuid> = uploads.iter().filter_map(|i| i.category_id).collect();
    let sub_category_ids: Vec<Uuid> = uploads.iter().filter_map(|i| i.sub_category_id).collect();
    let brand_ids: Vec<Uuid> = uploads.iter().filter_map(|i| i.brand_id).collect();

    let products = sqlx::query_as!(
        MiniProduct,
        "SELECT id, name FROM products WHERE id = ANY($1)",
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let categories = sqlx::query_as!(
        MiniCategory,
        "SELECT id, name FROM categories WHERE id = ANY($1)",
        &category_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sub_categories = sqlx::query_as!(
        MiniSubCategory,
        "SELECT id, name FROM sub_categories WHERE id = ANY($1)",
        &sub_category_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brands = sqlx::query_as!(
        MiniBrand,
        "SELECT id, name FROM brands WHERE id = ANY($1)",
        &brand_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = uploads
        .into_iter()
        .map(|upload| {
            let product = upload
                .product_id
                .and_then(|id| products.iter().find(|p| p.id == id))
                .cloned();
            let category = upload
                .category_id
                .and_then(|id| categories.iter().find(|c| c.id == id))
                .cloned();
            let sub_category = upload
                .sub_category_id
                .and_then(|id| sub_categories.iter().find(|s| s.id == id))
                .cloned();
            let brand = upload
                .brand_id
                .and_then(|id| brands.iter().find(|b| b.id == id))
                .cloned();

            UploadWithDetails {
                id: upload.id,
                product,
                category,
                sub_category,
                brand,
                name: upload.name,
                file_path: upload.file_path,
                file_type: upload.file_type,
                created_at: upload.created_at,
            }
        })
        .collect();

    Ok(result)
}

pub async fn get_uploads(
    State(state): State<AppState>,
) -> Result<Json<Vec<UploadWithDetails>>, StatusCode> {
    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uploads: Vec<Upload> = uploads.into_iter().map(Upload::with_full_url).collect();
    let result = attach_details(&state, uploads).await?;

    Ok(Json(result))
}

pub async fn get_upload(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<UploadWithDetails>, StatusCode> {
    let upload = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?
    .with_full_url();

    let result = attach_details(&state, vec![upload]).await?;
    let upload = result
        .into_iter()
        .next()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(upload))
}

pub async fn create_upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Upload>), StatusCode> {
    let mut product_id: Option<Uuid> = None;
    let mut category_id: Option<Uuid> = None;
    let mut sub_category_id: Option<Uuid> = None;
    let mut brand_id: Option<Uuid> = None;
    let mut name: Option<String> = None;
    let mut file_type: String = "image".to_string();
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut extension = String::from("bin");

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "product_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() {
                    product_id = Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?);
                }
            }
            "category_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() {
                    category_id =
                        Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?);
                }
            }
            "sub_category_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() {
                    sub_category_id =
                        Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?);
                }
            }
            "brand_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() {
                    brand_id = Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?);
                }
            }
            "name" => {
                name = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "file_type" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() {
                    file_type = text;
                }
            }
            "file" => {
                if let Some(filename) = field.file_name() {
                    if let Some(ext) = FsPath::new(filename).extension() {
                        extension = ext.to_string_lossy().to_string();
                    }
                }
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                file_bytes = Some(data.to_vec());
            }
            _ => {}
        }
    }

    let name = name.ok_or(StatusCode::BAD_REQUEST)?;
    let file_bytes = file_bytes.ok_or(StatusCode::BAD_REQUEST)?;

    fs::create_dir_all(UPLOAD_DIR)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let file_name = format!("{}.{}", slugify(&name, true), extension);
    let file_path = format!("{UPLOAD_DIR}/{file_name}");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    file.write_all(&file_bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let upload = sqlx::query_as!(
        Upload,
        r#"INSERT INTO uploads (product_id, category_id, sub_category_id, brand_id, name, file_path, file_type)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at"#,
        product_id,
        category_id,
        sub_category_id,
        brand_id,
        name,
        file_path,
        file_type
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
            StatusCode::BAD_REQUEST
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23514") => {
            StatusCode::BAD_REQUEST
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let upload = upload.with_full_url();
    Ok((StatusCode::CREATED, Json(upload)))
}

pub async fn update_upload(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<Upload>, StatusCode> {
    let existing = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let mut product_id: Option<Uuid> = existing.product_id;
    let mut category_id: Option<Uuid> = existing.category_id;
    let mut sub_category_id: Option<Uuid> = existing.sub_category_id;
    let mut brand_id: Option<Uuid> = existing.brand_id;
    let mut name: String = existing.name.clone();
    let mut file_type: String = existing.file_type.clone();
    let mut new_file_bytes: Option<Vec<u8>> = None;
    let mut extension = String::from("bin");

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "product_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                product_id = if text.is_empty() {
                    None
                } else {
                    Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?)
                };
            }
            "category_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                category_id = if text.is_empty() {
                    None
                } else {
                    Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?)
                };
            }
            "sub_category_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                sub_category_id = if text.is_empty() {
                    None
                } else {
                    Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?)
                };
            }
            "brand_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                brand_id = if text.is_empty() {
                    None
                } else {
                    Some(Uuid::parse_str(&text).map_err(|_| StatusCode::BAD_REQUEST)?)
                };
            }
            "name" => {
                name = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            }
            "file_type" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() {
                    file_type = text;
                }
            }
            "file" => {
                if let Some(filename) = field.file_name() {
                    if let Some(ext) = FsPath::new(filename).extension() {
                        extension = ext.to_string_lossy().to_string();
                    }
                }
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                new_file_bytes = Some(data.to_vec());
            }
            _ => {}
        }
    }

    let file_path = if let Some(bytes) = new_file_bytes {
        fs::create_dir_all(UPLOAD_DIR)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let file_name = format!("{}.{}", Uuid::new_v4(), extension);
        let new_path = format!("{UPLOAD_DIR}/{file_name}");

        let mut file = fs::File::create(&new_path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        file.write_all(&bytes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let _ = fs::remove_file(&existing.file_path).await;

        new_path
    } else {
        existing.file_path.clone()
    };

    let upload = sqlx::query_as!(
        Upload,
        r#"UPDATE uploads
           SET product_id = $1,
               category_id = $2,
               sub_category_id = $3,
               brand_id = $4,
               name = $5,
               file_path = $6,
               file_type = $7
           WHERE id = $8
           RETURNING id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at"#,
        product_id,
        category_id,
        sub_category_id,
        brand_id,
        name,
        file_path,
        file_type,
        uuid
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
            StatusCode::BAD_REQUEST
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23514") => {
            StatusCode::BAD_REQUEST
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let upload = upload.with_full_url();
    Ok(Json(upload))
}

pub async fn delete_upload(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let upload = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    sqlx::query!("DELETE FROM uploads WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = fs::remove_file(&upload.file_path).await;

    Ok(StatusCode::NO_CONTENT)
}
