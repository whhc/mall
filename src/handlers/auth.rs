use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::{models::dtos::RegisterUserDto, services};

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

pub async fn register(
    State(db): State<Arc<DatabaseConnection>>,
    Json(dto): Json<RegisterUserDto>,
) -> impl IntoResponse {
    if dto.email.is_empty() || dto.password.is_empty() || dto.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(ApiResponse {
                message: "Bad email or password or name".to_string(),
            }),
        );
    }

    match services::user::register(db, dto.email, dto.password, dto.name).await {
        Ok(user) => {
            tracing::info!("Register success: {:#?}", user);
            (
                StatusCode::CREATED,
                axum::Json(ApiResponse {
                    message: format!("User created success!"),
                }),
            )
        }
        Err(e) => {
            tracing::error!("Register error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                axum::Json(ApiResponse {
                    message: "Register error".to_string(),
                }),
            )
        }
    }
}
