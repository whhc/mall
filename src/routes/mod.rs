use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;

mod user;

pub fn create_routes(db: Arc<DatabaseConnection>) -> Router {
    Router::new().nest("/api/v1/user", user::routes(db))
}
