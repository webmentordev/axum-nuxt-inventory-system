use crate::AppState;
use crate::auth::*;
use crate::dashboard::contacts::*;

use anyhow::Result;
use axum::{Router, routing::post};

pub async fn init_public_route(state: AppState) -> Result<Router> {
    let users = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let contacts = Router::new().route("/", post(create_contact));

    let routes = Router::new()
        .nest("/users", users)
        .nest("/contacts", contacts)
        .with_state(state);

    Ok(routes)
}
