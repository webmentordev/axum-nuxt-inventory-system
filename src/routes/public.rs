use crate::AppState;
use crate::auth::*;
use crate::dashboard::contacts::*;
use crate::public::brands::*;
use crate::public::categories::*;
use crate::public::policies::*;
use crate::public::products::*;
use crate::public::sub_categories::*;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};

pub async fn init_public_route(state: AppState) -> Result<Router> {
    let users = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let contacts = Router::new().route("/", post(create_contact));

    let categories = Router::new()
        .route("/", get(get_public_categories))
        .route("/{slug}", get(get_public_category));

    let sub_categories = Router::new()
        .route("/", get(get_public_sub_categories))
        .route("/{slug}", get(get_public_sub_category));

    let brands = Router::new()
        .route("/", get(get_public_brands))
        .route("/{slug}", get(get_public_brand));

    let products = Router::new()
        .route("/", get(get_public_products))
        .route("/{slug}", get(get_public_product));

    let policies = Router::new().route("/{slug}", get(get_public_policy));

    let routes = Router::new()
        .nest("/users", users)
        .nest("/contacts", contacts)
        .nest("/categories", categories)
        .nest("/sub-categories", sub_categories)
        .nest("/brands", brands)
        .nest("/products", products)
        .nest("/policies", policies)
        .with_state(state);

    Ok(routes)
}
