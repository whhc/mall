use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use sea_orm::DatabaseConnection;

#[derive(Debug)]
pub struct AuthenticatedUser(pub String);

impl AuthenticatedUser {}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Arc<DatabaseConnection>: FromRef<S>,
    S: Send + Sync + std::fmt::Debug,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let _ = state;
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Authorization is missing".to_string(),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid Authorization header".to_string(),
                )
            })?;

        let cc = parts
            .headers
            .get("Cc")
            .ok_or((
                StatusCode::NON_AUTHORITATIVE_INFORMATION,
                "Cc is missing".to_string(),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    "Invalid cc header".to_string(),
                )
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "Authorization header format must be 'Bearer <token>'".to_string(),
        ))?;

        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWT secret not configured".to_string(),
            )
        })?;

        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            )
        })?;

        let decoded_cc = decoded.claims.sub;

        if &decoded_cc != cc {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization info ".to_string(),
            ));
        }

        Ok(AuthenticatedUser(cc.to_string()))
    }
}

#[derive(serde::Deserialize)]
struct Claims {
    sub: String,
    // exp: usize,
}
