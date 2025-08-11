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
    tracing::info!("Auth user is: {}", auth_user.0);
    match services::user::get_user(db, user_id, auth_user).await {
        Ok(user) => {
            tracing::info!("Get user info by user_id {user_id} success");
            (
                StatusCode::OK,
                axum::Json(ApiResponse {
                    message: "Success".to_string(),
                    user: Some(user),
                }),
            )
        }
        Err(e) => {
            tracing::error!("Get user info by user_id {user_id} error: {e}");
            (
                StatusCode::BAD_REQUEST,
                axum::Json(ApiResponse {
                    message: "Get user info error".to_string(),
                    user: None,
                }),
            )
        }
    }
}
