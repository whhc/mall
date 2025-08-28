use axum::Router;

use crate::models::app::AppState;

pub mod product;
mod user;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/user", user::routes())
        .nest("/api/v1", product::create_product_routes())
}
