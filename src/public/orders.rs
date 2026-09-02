use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::dashboard::orders::*;
use crate::utils::generate_order_number;

#[derive(Debug, Deserialize)]
pub struct CreatePublicOrderItem {
    pub slug: String,
    pub sku: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePublicOrder {
    pub user_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: String,
    pub shipping_address: String,
    pub notes: Option<String>,
    pub items: Vec<CreatePublicOrderItem>,
}

use axum::response::{IntoResponse, Response};

pub enum OrderError {
    EmptyCart,
    MissingField(&'static str),
    InvalidEmail,
    TooManyItems,
    InvalidQuantity,
    ProductUnavailable {
        sku: String,
    },
    InsufficientStock {
        product_name: String,
        available: i32,
    },
    OrderNotFound,
    Internal,
}

impl IntoResponse for OrderError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            OrderError::EmptyCart => (StatusCode::BAD_REQUEST, "Your cart is empty.".to_string()),
            OrderError::MissingField(field) => {
                (StatusCode::BAD_REQUEST, format!("{field} is required."))
            }
            OrderError::InvalidEmail => (
                StatusCode::BAD_REQUEST,
                "Please enter a valid email address.".to_string(),
            ),
            OrderError::TooManyItems => (
                StatusCode::BAD_REQUEST,
                "Too many items in the order.".to_string(),
            ),
            OrderError::InvalidQuantity => (
                StatusCode::BAD_REQUEST,
                "One or more items have an invalid quantity.".to_string(),
            ),
            OrderError::ProductUnavailable { sku } => (
                StatusCode::BAD_REQUEST,
                format!("Product '{sku}' is no longer available."),
            ),
            OrderError::InsufficientStock {
                product_name,
                available,
            } => (
                StatusCode::CONFLICT,
                format!("Only {available} unit(s) of '{product_name}' left in stock."),
            ),
            OrderError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.".to_string(),
            ),
            OrderError::OrderNotFound => (
                StatusCode::NOT_FOUND,
                "We couldn't find an order with that number.".to_string(),
            ),
        };

        (status, Json(serde_json::json!({ "message": message }))).into_response()
    }
}

pub async fn create_public_order(
    State(state): State<AppState>,
    Json(payload): Json<CreatePublicOrder>,
) -> Result<(StatusCode, Json<OrderWithItems>), OrderError> {
    if payload.items.is_empty() {
        return Err(OrderError::EmptyCart);
    }

    if payload.customer_name.trim().is_empty() {
        return Err(OrderError::MissingField("Name"));
    }
    if payload.customer_phone.trim().is_empty() {
        return Err(OrderError::MissingField("Phone number"));
    }
    if payload.shipping_address.trim().is_empty() {
        return Err(OrderError::MissingField("Shipping address"));
    }

    if let Some(email) = &payload.customer_email {
        if !email.is_empty() && !email.contains('@') {
            return Err(OrderError::InvalidEmail);
        }
    }

    if payload.items.len() > 100 {
        return Err(OrderError::TooManyItems);
    }

    let mut tx = state.db.begin().await.map_err(|_| OrderError::Internal)?;

    let mut subtotal = Decimal::ZERO;
    let mut resolved_items: Vec<(Uuid, String, String, Decimal, i32, Decimal)> = Vec::new();

    for item in &payload.items {
        if item.quantity <= 0 || item.quantity > 1000 {
            return Err(OrderError::InvalidQuantity);
        }

        let product = sqlx::query!(
            r#"SELECT id, name, sku, selling_price, quantity_in_stock
               FROM products
               WHERE slug = $1 AND sku = $2 AND is_active = true
               FOR UPDATE"#,
            item.slug,
            item.sku
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| OrderError::Internal)?
        .ok_or_else(|| OrderError::ProductUnavailable {
            sku: item.sku.clone(),
        })?;

        if product.quantity_in_stock < item.quantity {
            return Err(OrderError::InsufficientStock {
                product_name: product.name,
                available: product.quantity_in_stock,
            });
        }

        let line_total = product.selling_price * Decimal::from(item.quantity);
        subtotal += line_total;

        resolved_items.push((
            product.id,
            product.name,
            product.sku,
            product.selling_price,
            item.quantity,
            line_total,
        ));

        sqlx::query!(
            "UPDATE products SET quantity_in_stock = quantity_in_stock - $1 WHERE id = $2",
            item.quantity,
            product.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| OrderError::Internal)?;
    }

    let tax_amount = Decimal::ZERO;
    let shipping_amount = Decimal::ZERO;
    let total_amount = subtotal + tax_amount + shipping_amount;

    let mut order = None;
    for _ in 0..5 {
        let order_number = generate_order_number();

        let attempt = sqlx::query_as!(
            Order,
            r#"INSERT INTO orders (order_number, user_id, customer_name, customer_email, customer_phone,
                                    shipping_address, notes, status, subtotal, tax_amount, shipping_amount, total_amount)
               VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', $8, $9, $10, $11)
               RETURNING id, order_number, user_id, customer_name, customer_email, customer_phone,
                         shipping_address, status as "status: OrderStatus", subtotal, tax_amount,
                         shipping_amount, total_amount, notes, created_at, updated_at"#,
            order_number,
            payload.user_id,
            payload.customer_name.trim(),
            payload.customer_email.as_deref().map(|s| s.trim()),
            payload.customer_phone.trim(),
            payload.shipping_address.trim(),
            payload.notes.as_deref().map(|s| s.trim()),
            subtotal,
            tax_amount,
            shipping_amount,
            total_amount
        )
        .fetch_one(&mut *tx)
        .await;

        match attempt {
            Ok(o) => {
                order = Some(o);
                break;
            }
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                continue;
            }
            Err(_) => return Err(OrderError::Internal),
        }
    }

    let order = order.ok_or(OrderError::Internal)?;

    let mut items = Vec::with_capacity(resolved_items.len());

    for (product_id, product_name, product_sku, unit_price, quantity, line_total) in resolved_items
    {
        let order_item = sqlx::query_as!(
            OrderItem,
            r#"INSERT INTO order_items (order_id, product_id, product_name, product_sku, unit_price, quantity, line_total)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id, order_id, product_id, product_name, product_sku, unit_price, quantity, line_total,
                         status as "status: OrderItemStatus", created_at"#,
            order.id,
            product_id,
            product_name,
            product_sku,
            unit_price,
            quantity,
            line_total
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| OrderError::Internal)?;

        items.push(order_item);
    }

    tx.commit().await.map_err(|_| OrderError::Internal)?;

    Ok((StatusCode::CREATED, Json(OrderWithItems { order, items })))
}

pub async fn track_public_order(
    State(state): State<AppState>,
    Path(order_number): Path<String>,
) -> Result<Json<OrderWithItems>, OrderError> {
    let order_number = order_number.trim();

    if order_number.is_empty() {
        return Err(OrderError::MissingField("Order number"));
    }

    let mut tx = state.db.begin().await.map_err(|_| OrderError::Internal)?;

    let order = sqlx::query_as!(
        Order,
        r#"SELECT id, order_number, user_id, customer_name, customer_email, customer_phone,
                  shipping_address, status as "status: OrderStatus", subtotal, tax_amount,
                  shipping_amount, total_amount, notes, created_at, updated_at
           FROM orders
           WHERE order_number = $1"#,
        order_number
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| OrderError::Internal)?
    .ok_or(OrderError::OrderNotFound)?;

    let items = sqlx::query_as!(
        OrderItem,
        r#"SELECT id, order_id, product_id, product_name, product_sku, unit_price, quantity, line_total,
                  status as "status: OrderItemStatus", created_at
           FROM order_items
           WHERE order_id = $1
           ORDER BY created_at ASC"#,
        order.id
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| OrderError::Internal)?;

    tx.commit().await.map_err(|_| OrderError::Internal)?;

    Ok(Json(OrderWithItems { order, items }))
}
