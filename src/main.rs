use std::net::SocketAddr;

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use routes::create_routes;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use models::app::AppState;

mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("mall=debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .unwrap();

    tracing::info!("Connecting to database: {}", database_url);
    let db = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to database.");

    tracing::info!("Running database migrations...");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations.");
    tracing::info!("Migrations completed successfully");

    // 创建应用状态
    let app_state = AppState { db, jwt_secret };

    // 创建路由并传入状态
    let app = create_routes().with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Available API endpoints:");
    tracing::info!("  - GET    /api/v1/products           - 获取产品列表");
    tracing::info!("  - GET    /api/v1/products/{{id}}      - 获取产品详情");
    tracing::info!("  - POST   /api/v1/products           - 创建产品（管理员）");
    tracing::info!("  - PUT    /api/v1/products/{{id}}      - 更新产品（管理员）");
    tracing::info!("  - DELETE /api/v1/products/{{id}}      - 删除产品（管理员）");
    tracing::info!("  - GET    /api/v1/categories         - 获取分类列表");
    tracing::info!("  - POST   /api/v1/categories         - 创建分类（管理员）");
    tracing::info!("  - GET    /api/v1/regions            - 获取地区列表");
    tracing::info!("  - POST   /api/v1/regions            - 创建地区（管理员）");
    tracing::info!("  - POST   /api/v1/user/register   - 用户注册");
    tracing::info!("  - POST   /api/v1/user/login      - 用户登录");
    tracing::info!("  - GET    /api/v1/user/profile/{{id}} - 获取用户信息");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

pub async fn health_check() -> &'static str {
    "OK"
}
