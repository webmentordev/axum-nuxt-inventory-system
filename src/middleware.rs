use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{AppState, auth::Claims};

async fn authenticate(state: &AppState, headers: &HeaderMap) -> Result<Claims, StatusCode> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .claims;

    let user = sqlx::query!(
        "SELECT token_version, is_active, is_admin FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    if !user.is_active || user.token_version != claims.ver {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Claims {
        is_admin: user.is_admin,
        ..claims
    })
}

pub async fn require_admin(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = authenticate(&state, req.headers()).await?;

    if !claims.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = authenticate(&state, req.headers()).await?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
