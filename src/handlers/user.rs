use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::{middlewares::auth::AuthenticatedUser, services};

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    user: Option<entity::user::Model>,
}

pub async fn get_profile(
    State(db): State<Arc<DatabaseConnection>>,
    Path(user_id): Path<i32>,
    auth_user: AuthenticatedUser,
) -> impl IntoResponse {
    match services::user::get_user(db, user_id, auth_user).await {
        Ok(user) => (
            StatusCode::OK,
            axum::Json(ApiResponse {
                message: "Success".to_string(),
                user: Some(user),
            }),
        ),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            axum::Json(ApiResponse {
                message: "Get user info error".to_string(),
                user: None,
            }),
        ),
    }
}
