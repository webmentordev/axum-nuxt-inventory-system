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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Image {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub sub_category_id: Option<Uuid>,
    pub name: String,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
}

const UPLOAD_DIR: &str = "uploads/images";

pub async fn create_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Image>), StatusCode> {
    let mut product_id: Option<Uuid> = None;
    let mut category_id: Option<Uuid> = None;
    let mut sub_category_id: Option<Uuid> = None;
    let mut name: Option<String> = None;
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
            "name" => {
                name = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
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

    let file_name = format!("{}.{}", Uuid::new_v4(), extension);
    let file_path = format!("{UPLOAD_DIR}/{file_name}");

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    file.write_all(&file_bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let image = sqlx::query_as!(
        Image,
        r#"INSERT INTO images (product_id, category_id, sub_category_id, name, file_path)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, product_id, category_id, sub_category_id, name, file_path, created_at"#,
        product_id,
        category_id,
        sub_category_id,
        name,
        file_path
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
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::CREATED, Json(image)))
}

pub async fn get_images(State(state): State<AppState>) -> Result<Json<Vec<Image>>, StatusCode> {
    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, name, file_path, created_at
           FROM images
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(images))
}

pub async fn get_image(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Image>, StatusCode> {
    let image = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, name, file_path, created_at
           FROM images
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(image))
}

pub async fn update_image(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<Image>, StatusCode> {
    let existing = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, name, file_path, created_at
           FROM images
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
    let mut name: String = existing.name.clone();
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
            "name" => {
                name = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
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

    let image = sqlx::query_as!(
        Image,
        r#"UPDATE images
           SET product_id = $1,
               category_id = $2,
               sub_category_id = $3,
               name = $4,
               file_path = $5
           WHERE id = $6
           RETURNING id, product_id, category_id, sub_category_id, name, file_path, created_at"#,
        product_id,
        category_id,
        sub_category_id,
        name,
        file_path,
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
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(image))
}

pub async fn delete_image(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let image = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, name, file_path, created_at
           FROM images
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    sqlx::query!("DELETE FROM images WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = fs::remove_file(&image.file_path).await;

    Ok(StatusCode::NO_CONTENT)
}
