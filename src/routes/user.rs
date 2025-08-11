use std::sync::Arc;

use axum::routing::post;
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::handlers::{auth, user};

pub fn routes(db: Arc<DatabaseConnection>) -> Router {
    // let auth_route = Router::new().with_state(db.clone());

    Router::new()
        .route("/register", post(auth::register))
        // .route("/send-code", post(user::send_code))
        .route("/login", post(auth::login))
        // .nest(
        //     "/profile",
        //     Router::new()
        //         .route("/{userId}", get(user::get_profile))
        //         .route_layer(axum::middleware::from_extractor::<AuthenticatedUser>()),
        // )
        .route("/profile/{userId}", get(user::get_profile))
        // .route("/refresh", post(user::refresh))
        // .route("/reset-password", post(user::reset_post))
        // .route("/profile", put(user::change_profile))
        // .route("/password", put(user::change_password))
        .with_state(db)
}
