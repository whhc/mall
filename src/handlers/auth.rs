use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::{
    models::dtos::{LoginUserDto, RegisterUserDto},
    services,
};

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    cc: Option<String>,
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
                cc: None,
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
                    cc: None,
                }),
            )
        }
        Err(e) => {
            tracing::error!("Register error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                axum::Json(ApiResponse {
                    message: "Register error".to_string(),
                    cc: None,
                }),
            )
        }
    }
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    code: i32,
    cc: Option<String>,
    message: String,
    token: Option<String>,
}

pub async fn login(
    State(db): State<Arc<DatabaseConnection>>,
    Json(dto): Json<LoginUserDto>,
) -> impl IntoResponse {
    if dto.email.is_empty() || dto.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            axum::Json(LoginResponse {
                code: 1,
                message: "Email or password should not be empty".to_string(),
                cc: None,
                token: None,
            }),
        );
    }

    match services::auth::login(db, &dto.email, &dto.password).await {
        Ok(info) => {
            tracing::info!("Login succecc with token: {}", info.token);
            (
                StatusCode::OK,
                axum::Json(LoginResponse {
                    code: 0,
                    message: "success".to_string(),
                    cc: Some(info.user_id),
                    token: Some(info.token),
                }),
            )
        }
        Err(e) => {
            tracing::error!("Login error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                axum::Json(LoginResponse {
                    code: 1,
                    message: format!("Login Error: {}", e),
                    cc: None,
                    token: None,
                }),
            )
        }
    }
}
