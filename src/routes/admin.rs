use crate::AppState;
use crate::auth::*;
use crate::dashboard::barcodes::*;
use crate::dashboard::brands::*;
use crate::dashboard::categories::*;
use crate::dashboard::contacts::*;
use crate::dashboard::orders::*;
use crate::dashboard::policies::*;
use crate::dashboard::products::*;
use crate::dashboard::products_seo::*;
use crate::dashboard::stats::*;
use crate::dashboard::sub_categories::*;
use crate::dashboard::uploads::*;
use crate::middleware::require_admin;
use axum::middleware as axum_middleware;

use anyhow::Result;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub async fn init_admin_routes(state: AppState) -> Result<Router> {
    let product = Router::new()
        .route("/", get(get_products).post(create_product))
        .route("/list", get(get_products_list))
        .route("/{uuid}", get(get_product).patch(update_product));

    let categories = Router::new()
        .route("/", get(get_categories).post(create_category))
        .route(
            "/{uuid}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        );

    let sub_categories = Router::new()
        .route("/", get(get_sub_categories).post(create_sub_category))
        .route("/by-category/{uuid}", get(get_sub_categories_by_category))
        .route(
            "/{uuid}",
            get(get_sub_category)
                .patch(update_sub_category)
                .delete(delete_sub_category),
        );

    let orders = Router::new()
        .route("/", get(get_orders).post(create_order))
        .route(
            "/{order_id}",
            get(get_order).patch(update_order).delete(delete_order),
        )
        .route(
            "/{order_id}/items",
            get(get_order_items).post(add_order_items),
        )
        .route(
            "/{order_id}/items/{item_id}",
            get(get_order_item)
                .patch(update_order_item)
                .delete(delete_order_item),
        )
        .route(
            "/{order_id}/items/{item_id}/status",
            patch(update_order_item_status),
        );

    let uploads = Router::new()
        .route("/", get(get_uploads).post(create_upload))
        .route(
            "/{uuid}",
            get(get_upload).patch(update_upload).delete(delete_upload),
        );

    let barcodes = Router::new()
        .route("/", get(get_barcodes).post(create_barcode))
        .route("/bulk", post(create_barcodes_bulk))
        .route(
            "/{uuid}",
            get(get_barcode)
                .patch(update_barcode)
                .delete(delete_barcode),
        );

    let users = Router::new().route("/", get(get_users)).route(
        "/{uuid}",
        get(get_user).patch(update_user).delete(delete_user),
    );

    let contacts = Router::new()
        .route("/", get(get_contacts))
        .route("/{uuid}", delete(delete_contact));

    let brands = Router::new()
        .route("/", get(get_brands).post(create_brand))
        .route(
            "/{uuid}",
            get(get_brand).patch(update_brand).delete(delete_brand),
        );

    let stats = Router::new().route("/", get(get_dashboard_stats));

    let seo = Router::new()
        .route("/", get(get_products_seo).post(create_product_seo))
        .route(
            "/{uuid}",
            get(get_product_seo)
                .patch(update_product_seo)
                .delete(delete_product_seo),
        );

    let policies = Router::new()
        .route("/", get(get_policies).post(create_policy))
        .route("/{uuid}", patch(update_policy).delete(delete_policy));

    let routes = Router::new()
        .nest("/products", product)
        .nest("/categories", categories)
        .nest("/sub-categories", sub_categories)
        .nest("/orders", orders)
        .nest("/uploads", uploads)
        .nest("/barcodes", barcodes)
        .nest("/users", users)
        .nest("/contacts", contacts)
        .nest("/brands", brands)
        .nest("/stats", stats)
        .nest("/seo", seo)
        .nest("/policies", policies)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            require_admin,
        ))
        .with_state(state);

    Ok(routes)
}
