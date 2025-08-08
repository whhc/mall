use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post, put};
use sea_orm::DatabaseConnection;

use crate::handlers::auth;

pub fn routes(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/register", post(auth::register))
        // .route("/send-code", post(user::send_code))
        // .route("/login", post(user::login))
        // .route("/refresh", post(user::refresh))
        // .route("/reset-password", post(user::reset_post))
        // .route("/profile", get(user::get_profile))
        // .route("/profile", put(user::change_profile))
        // .route("/password", put(user::change_password))
        .with_state(db)
}
