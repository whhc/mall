use axum::{Router, routing::get};

use crate::handlers::product;
use crate::models::app::AppState;

/// 创建产品管理相关的路由
///
/// API路由结构：
/// - GET    /api/products           - 获取产品列表（支持分页和筛选）
/// - GET    /api/products/{id}      - 获取单个产品详情
/// - POST   /api/products           - 创建新产品（仅管理员）
/// - PUT    /api/products/{id}      - 更新产品信息（仅管理员）
/// - DELETE /api/products/{id}      - 删除产品（仅管理员）
/// - GET    /api/categories         - 获取分类列表
/// - POST   /api/categories         - 创建新分类（仅管理员）
/// - GET    /api/regions            - 获取地区列表
/// - POST   /api/regions            - 创建新地区（仅管理员）
pub fn create_product_routes() -> Router<AppState> {
    Router::new()
        // 产品相关路由
        .route(
            "/products",
            get(product::list_products).post(product::create_product),
        )
        .route(
            "/products/{id}",
            get(product::get_product)
                .put(product::update_product)
                .delete(product::delete_product),
        )
        // 分类相关路由
        .route(
            "/categories",
            get(product::list_categories).post(product::create_category),
        )
        // 地区相关路由
        .route(
            "/regions",
            get(product::list_regions).post(product::create_region),
        )
}
