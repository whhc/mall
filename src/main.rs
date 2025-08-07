use std::net::SocketAddr;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("mall=debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{database_url}");
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .unwrap();

    let connection = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to database.");
    Migrator::up(&connection, None)
        .await
        .expect("Failed to run migrations.");

    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(connection);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    tracing::info!("Server runing on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
