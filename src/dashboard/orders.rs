use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::utils::generate_order_number;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Walkin,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,

    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub shipping_address: Option<String>,

    pub status: OrderStatus,

    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub shipping_amount: Decimal,
    pub total_amount: Decimal,

    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,

    pub product_name: String,
    pub product_sku: String,
    pub unit_price: Decimal,
    pub quantity: i32,
    pub line_total: Decimal,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub status: Option<OrderStatus>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrder {
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub shipping_address: Option<String>,
    pub status: Option<OrderStatus>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderWithItems {
    #[serde(flatten)]
    pub order: Order,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Serialize)]
pub struct OrderWithItemCount {
    #[serde(flatten)]
    pub order: Order,
    pub total_items: i64,
}

#[derive(Debug, Serialize)]
pub struct ItemWithOrder {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_sku: String,
    pub unit_price: Decimal,
    pub quantity: i32,
    pub line_total: Decimal,
    pub created_at: DateTime<Utc>,

    pub order: Order,
}

#[derive(Debug, Deserialize)]
pub struct AddOrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct AddOrderItems {
    pub items: Vec<AddOrderItem>,
}

pub async fn get_orders(
    State(state): State<AppState>,
) -> Result<Json<Vec<OrderWithItemCount>>, StatusCode> {
    let rows = sqlx::query!(
        r#"SELECT o.id, o.order_number, o.customer_name, o.customer_email, o.customer_phone,
                  o.shipping_address, o.status as "status: OrderStatus", o.subtotal, o.tax_amount,
                  o.shipping_amount, o.total_amount, o.notes, o.created_at, o.updated_at,
                  COALESCE(SUM(oi.quantity), 0)::bigint as "total_items!"
           FROM orders o
           LEFT JOIN order_items oi ON oi.order_id = o.id
           GROUP BY o.id
           ORDER BY o.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let orders = rows
        .into_iter()
        .map(|r| OrderWithItemCount {
            order: Order {
                id: r.id,
                order_number: r.order_number,
                customer_name: r.customer_name,
                customer_email: r.customer_email,
                customer_phone: r.customer_phone,
                shipping_address: r.shipping_address,
                status: r.status,
                subtotal: r.subtotal,
                tax_amount: r.tax_amount,
                shipping_amount: r.shipping_amount,
                total_amount: r.total_amount,
                notes: r.notes,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            total_items: r.total_items,
        })
        .collect();

    Ok(Json(orders))
}

pub async fn get_order_items(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Vec<ItemWithOrder>>, StatusCode> {
    let order = sqlx::query_as!(
        Order,
        r#"SELECT id, order_number, customer_name, customer_email, customer_phone,
                  shipping_address, status as "status: OrderStatus", subtotal, tax_amount,
                  shipping_amount, total_amount, notes, created_at, updated_at
           FROM orders
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let items = sqlx::query_as!(
        OrderItem,
        r#"SELECT id, order_id, product_id, product_name, product_sku, unit_price, quantity, line_total, created_at
           FROM order_items
           WHERE order_id = $1
           ORDER BY created_at ASC"#,
        uuid
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items_with_order = items
        .into_iter()
        .map(|item| ItemWithOrder {
            id: item.id,
            order_id: item.order_id,
            product_id: item.product_id,
            product_name: item.product_name,
            product_sku: item.product_sku,
            unit_price: item.unit_price,
            quantity: item.quantity,
            line_total: item.line_total,
            created_at: item.created_at,
            order: order.clone(),
        })
        .collect();

    Ok(Json(items_with_order))
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrder>,
) -> Result<(StatusCode, Json<Order>), StatusCode> {
    if payload.customer_name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let status = payload.status.unwrap_or(OrderStatus::Pending);

    let mut order = None;
    for _ in 0..5 {
        let order_number = generate_order_number();

        let attempt = sqlx::query_as!(
            Order,
            r#"INSERT INTO orders (order_number, customer_name, customer_email, customer_phone,
                                    shipping_address, notes, status, subtotal, tax_amount, shipping_amount, total_amount)
               VALUES ($1, $2, $3, $4, $5, $6, $7, 0, 0, 0, 0)
               RETURNING id, order_number, customer_name, customer_email, customer_phone,
                         shipping_address, status as "status: OrderStatus", subtotal, tax_amount,
                         shipping_amount, total_amount, notes, created_at, updated_at"#,
            order_number,
            payload.customer_name,
            payload.customer_email,
            payload.customer_phone,
            payload.shipping_address,
            payload.notes,
            status as OrderStatus
        )
        .fetch_one(&state.db)
        .await;

        match attempt {
            Ok(o) => {
                order = Some(o);
                break;
            }
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                continue;
            }
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    let order = order.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn get_order(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<OrderWithItems>, StatusCode> {
    let order = sqlx::query_as!(
        Order,
        r#"SELECT id, order_number, customer_name, customer_email, customer_phone,
                  shipping_address, status as "status: OrderStatus", subtotal, tax_amount,
                  shipping_amount, total_amount, notes, created_at, updated_at
           FROM orders
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let items = sqlx::query_as!(
        OrderItem,
        r#"SELECT id, order_id, product_id, product_name, product_sku, unit_price, quantity, line_total, created_at
           FROM order_items
           WHERE order_id = $1
           ORDER BY created_at ASC"#,
        uuid
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(OrderWithItems { order, items }))
}

pub async fn update_order(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateOrder>,
) -> Result<Json<Order>, StatusCode> {
    let order = sqlx::query_as!(
        Order,
        r#"UPDATE orders
           SET customer_name = COALESCE($1, customer_name),
               customer_email = COALESCE($2, customer_email),
               customer_phone = COALESCE($3, customer_phone),
               shipping_address = COALESCE($4, shipping_address),
               status = COALESCE($5, status),
               notes = COALESCE($6, notes),
               updated_at = NOW()
           WHERE id = $7
           RETURNING id, order_number, customer_name, customer_email, customer_phone,
                     shipping_address, status as "status: OrderStatus", subtotal, tax_amount,
                     shipping_amount, total_amount, notes, created_at, updated_at"#,
        payload.customer_name,
        payload.customer_email,
        payload.customer_phone,
        payload.shipping_address,
        payload.status as Option<OrderStatus>,
        payload.notes,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(order))
}

pub async fn delete_order(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM orders WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_order_items(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<AddOrderItems>,
) -> Result<(StatusCode, Json<OrderWithItems>), StatusCode> {
    if payload.items.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!("SELECT id FROM orders WHERE id = $1 FOR UPDATE", uuid)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut added_total = Decimal::ZERO;
    let mut inserted_items = Vec::new();

    for item in &payload.items {
        if item.quantity <= 0 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let product = sqlx::query!(
            r#"SELECT name, sku, selling_price, quantity_in_stock
               FROM products
               WHERE id = $1
               FOR UPDATE"#,
            item.product_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)?;

        if product.quantity_in_stock < item.quantity {
            return Err(StatusCode::CONFLICT);
        }

        let line_total = product.selling_price * Decimal::from(item.quantity);
        added_total += line_total;

        sqlx::query!(
            "UPDATE products SET quantity_in_stock = quantity_in_stock - $1 WHERE id = $2",
            item.quantity,
            item.product_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let order_item = sqlx::query_as!(
            OrderItem,
            r#"INSERT INTO order_items (order_id, product_id, product_name, product_sku, unit_price, quantity, line_total)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id, order_id, product_id, product_name, product_sku, unit_price, quantity, line_total, created_at"#,
            uuid,
            item.product_id,
            product.name,
            product.sku,
            product.selling_price,
            item.quantity,
            line_total
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        inserted_items.push(order_item);
    }

    let order = sqlx::query_as!(
        Order,
        r#"UPDATE orders
           SET subtotal = subtotal + $1,
               total_amount = total_amount + $1,
               updated_at = NOW()
           WHERE id = $2
           RETURNING id, order_number, customer_name, customer_email, customer_phone,
                     shipping_address, status as "status: OrderStatus", subtotal, tax_amount,
                     shipping_amount, total_amount, notes, created_at, updated_at"#,
        added_total,
        uuid
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(OrderWithItems {
            order,
            items: inserted_items,
        }),
    ))
}
