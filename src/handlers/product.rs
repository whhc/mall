use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::middlewares::auth::{AuthenticatedUser, OptionalAuthenticatedUser};
use crate::models::{
    app::{ApiResponse, AppState},
    dtos::*,
};
use crate::services::product as product_service;

pub async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<ProductListParams>,
    OptionalAuthenticatedUser(user): OptionalAuthenticatedUser,
) -> impl IntoResponse {
    match product_service::list_products(&state.db, params, user.as_ref()).await {
        Ok(response) => (StatusCode::OK, Json(ApiResponse::success(response))).into_response(),
        Err(e) => {
            tracing::error!("Failed to get product list: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to get product list")),
            )
                .into_response()
        }
    }
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
    OptionalAuthenticatedUser(user): OptionalAuthenticatedUser,
) -> impl IntoResponse {
    match product_service::get_product(&state.db, product_id, user.as_ref()).await {
        Ok(response) => (StatusCode::OK, Json(ApiResponse::success(response))).into_response(),
        Err(e) => {
            tracing::error!("Failed to get product: {}", e);
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("Product not found")),
            )
                .into_response()
        }
    }
}

pub async fn create_product(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(request): Json<CreateProductRequest>,
) -> impl IntoResponse {
    tracing::info!("Creating product");

    if !user.can_modify_products() {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("Permission denied")),
        )
            .into_response();
    }

    match product_service::create_product(&state.db, request, user).await {
        Ok(product_id) => (
            StatusCode::CREATED,
            Json(ApiResponse::success_with_message(
                json!({ "product_id": product_id }),
                "Product created successfully",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to create product: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to create product")),
            )
                .into_response()
        }
    }
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
    user: AuthenticatedUser,
    Json(request): Json<UpdateProductRequest>,
) -> impl IntoResponse {
    if !user.can_modify_products() {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("Permission denied")),
        )
            .into_response();
    }

    match product_service::update_product(&state.db, product_id, request).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success_with_message(
                json!({}),
                "Product updated successfully",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to update product: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to update product")),
            )
                .into_response()
        }
    }
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    if !user.can_modify_products() {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("Permission denied")),
        )
            .into_response();
    }

    match product_service::delete_product(&state.db, product_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success_with_message(
                json!({}),
                "Product deleted successfully",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to delete product: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to delete product")),
            )
                .into_response()
        }
    }
}

pub async fn list_categories(State(state): State<AppState>) -> impl IntoResponse {
    match product_service::list_categories(&state.db).await {
        Ok(response) => (StatusCode::OK, Json(ApiResponse::success(response))).into_response(),
        Err(e) => {
            tracing::error!("Failed to get categories: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to get categories")),
            )
                .into_response()
        }
    }
}

pub async fn create_category(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(request): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    if !user.can_manage_categories_regions() {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("Permission denied")),
        )
            .into_response();
    }

    match product_service::create_category(&state.db, request).await {
        Ok(category_id) => (
            StatusCode::CREATED,
            Json(ApiResponse::success_with_message(
                json!({ "category_id": category_id }),
                "Category created successfully",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to create category: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to create category")),
            )
                .into_response()
        }
    }
}

pub async fn list_regions(State(state): State<AppState>) -> impl IntoResponse {
    match product_service::list_regions(&state.db).await {
        Ok(response) => (StatusCode::OK, Json(ApiResponse::success(response))).into_response(),
        Err(e) => {
            tracing::error!("Failed to get regions: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to get regions")),
            )
                .into_response()
        }
    }
}

pub async fn create_region(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(request): Json<CreateRegionRequest>,
) -> impl IntoResponse {
    if !user.can_manage_categories_regions() {
        return (
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("Permission denied")),
        )
            .into_response();
    }

    match product_service::create_region(&state.db, request).await {
        Ok(region_id) => (
            StatusCode::CREATED,
            Json(ApiResponse::success_with_message(
                json!({ "region_id": region_id }),
                "Region created successfully",
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to create region: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Failed to create region")),
            )
                .into_response()
        }
    }
}
