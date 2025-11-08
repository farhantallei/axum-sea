use crate::{
    modules::shared::error::AppError,
    state::AppState,
    utils::auth::{Claims, verify_token},
};
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};

pub struct AuthClaims(pub Claims);

impl FromRequestParts<AppState> for AuthClaims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization scheme".into()))?;

        let claims = verify_token(&state.jwt_config, token)
            .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

        Ok(AuthClaims(claims))
    }
}
