use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

use crate::{
    middlewares::auth::AuthenticatedUser,
    models::{app::AppState, dtos::UpdateUserRequest},
    services,
};

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
    pub user: Option<entity::user::PartialUser>,
}

pub async fn get_profile(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    auth_user: AuthenticatedUser,
) -> impl IntoResponse {
    match services::user::get_user(&state.db, user_id, auth_user).await {
        Ok(user) => (
            StatusCode::OK,
            axum::Json(ApiResponse {
                message: "Success".to_string(),
                user: Some(user),
            }),
        ),
        Err(e) => {
            tracing::error!("{:?}", e);
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(ApiResponse {
                    message: "Get user info error".to_string(),
                    user: None,
                }),
            );
        }
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    _user: AuthenticatedUser,
    Json(request): Json<UpdateUserRequest>,
) -> (StatusCode, axum::Json<ApiResponse>) {
    match services::user::update_user(&state.db, user_id, request).await {
        Ok(user) => (
            StatusCode::OK,
            axum::Json(ApiResponse {
                message: "Update user info success".to_string(),
                user: Some(user),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ApiResponse {
                message: "Update user info error".to_string(),
                user: None,
            }),
        ),
    }
}
