use axum::routing::post;
use axum::{Router, routing::get};

use crate::handlers::{auth, user};
use crate::models::app::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route(
            "/profile/{userId}",
            get(user::get_profile).put(user::update_user),
        )
}
